use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use music_things_pitch::stft::{StftConfig, StftPitchShift, StftPitchShiftStream};

struct CountingAllocator;
static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

#[test]
fn processing_and_factor_updates_do_not_allocate() {
    let mut config = StftConfig::symmetric(1024, 256, 44100.0);
    config.max_factors = 2;
    let mut offline = StftPitchShift::new(config).unwrap();
    let mut stream = StftPitchShiftStream::new(config).unwrap();
    let input = vec![0.1; 4096];
    let mut output = vec![0.0; input.len()];
    ALLOCATIONS.store(0, Ordering::SeqCst);
    COUNTING.store(true, Ordering::SeqCst);
    let offline_result = offline.set_factors(&[0.75, 1.25]);
    let stream_result = stream.set_factors(&[0.75, 1.25]);
    offline.shift_pitch(&input, &mut output);
    for (source, destination) in input.chunks(128).zip(output.chunks_mut(128)) {
        stream.process(source, destination);
    }
    COUNTING.store(false, Ordering::SeqCst);
    let count = ALLOCATIONS.load(Ordering::SeqCst);
    assert!(offline_result.is_ok() && stream_result.is_ok());
    assert_eq!(count, 0, "processing allocated {count} times");
}
