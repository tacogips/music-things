use music_things_pitch::dywa::PitchTracker;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct Counting;
static ENABLED: AtomicBool = AtomicBool::new(false);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if ENABLED.load(Ordering::Relaxed) {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if ENABLED.load(Ordering::Relaxed) {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[test]
fn processing_does_not_allocate() {
    let mut tracker = PitchTracker::new(44100.0, 1024, 256).unwrap();
    let samples = [0.1_f32; 2048];
    ALLOCS.store(0, Ordering::SeqCst);
    ENABLED.store(true, Ordering::SeqCst);
    std::hint::black_box(tracker.compute(&samples));
    std::hint::black_box(tracker.push(&samples));
    tracker.reset();
    ENABLED.store(false, Ordering::SeqCst);
    assert_eq!(ALLOCS.load(Ordering::SeqCst), 0);
}
