use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
struct Counting;
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::SeqCst);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::SeqCst);
        System.realloc(ptr, layout, size)
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;
#[test]
fn realtime_methods_do_not_allocate() {
    for info in music_things_airwindows::catalog() {
        let mut e = music_things_airwindows::create(info.name, 44100.0).unwrap();
        let input = [0.0; 128];
        let mut l = [0.0; 128];
        let mut r = [0.0; 128];
        e.process(&input, &input, &mut l, &mut r);
        let before = ALLOCS.load(Ordering::SeqCst);
        e.process(&input, &input, &mut l, &mut r);
        for i in 0..e.param_count() {
            let p = e.get_param(i);
            e.set_param(i, p);
        }
        e.reset();
        let after = ALLOCS.load(Ordering::SeqCst);
        assert_eq!(after - before, 0, "{} allocated", info.name);
    }
}
