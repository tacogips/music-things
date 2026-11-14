#[path = "common/sf2_builder.rs"]
mod sf2_builder;

use music_things_soundfont::{
    font::SoundFont,
    synth::{Synth, SynthConfig},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

struct CountingAllocator;
static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        System.realloc(ptr, layout, size)
    }
}
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

#[test]
fn realtime_calls_do_not_allocate() {
    let font = Arc::new(SoundFont::load(&sf2_builder::synthetic_sf2()).unwrap());
    let mut synth = Synth::new(
        font,
        SynthConfig {
            max_voices: 8,
            ..SynthConfig::default()
        },
    );
    let mut out = [0.0; 128];
    ALLOCATIONS.store(0, Ordering::SeqCst);
    COUNTING.store(true, Ordering::SeqCst);
    synth.channel_set_presetnumber(0, 0, false);
    for key in 48..=64 {
        synth.note_on(0, key, 0.8);
        synth.channel_note_on(0, key, 0.8);
    }
    synth.channel_set_pan(0, 0.2);
    synth.channel_set_volume(0, 0.5);
    synth.channel_set_pitchrange(0, 12.0);
    synth.channel_set_pitchwheel(0, 12000);
    synth.channel_midi_control(0, 64, 127);
    synth.render_float(&mut out, 64, false);
    synth.channel_note_off(0, 60);
    synth.channel_note_off_all(0);
    synth.channel_sounds_off_all(0);
    synth.note_off_all();
    synth.reset();
    COUNTING.store(false, Ordering::SeqCst);
    assert_eq!(ALLOCATIONS.load(Ordering::SeqCst), 0);
}
