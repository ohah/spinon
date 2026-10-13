use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static FAIL_NEXT_ALLOCATION: Cell<bool> = const { Cell::new(false) };
    static FAILURE_OBSERVED: Cell<bool> = const { Cell::new(false) };
}

struct FailOnceAllocator;

#[global_allocator]
static ALLOCATOR: FailOnceAllocator = FailOnceAllocator;

pub(crate) fn fail_next_allocation() {
    FAIL_NEXT_ALLOCATION.with(|armed| armed.set(true));
}

pub(crate) fn take_failure_observed() -> bool {
    FAILURE_OBSERVED.with(|observed| observed.replace(false))
}

pub(crate) fn reset() {
    FAIL_NEXT_ALLOCATION.with(|armed| armed.set(false));
    FAILURE_OBSERVED.with(|observed| observed.set(false));
}

fn should_fail() -> bool {
    let fail = FAIL_NEXT_ALLOCATION
        .try_with(|armed| armed.replace(false))
        .unwrap_or(false);
    if fail {
        let _ = FAILURE_OBSERVED.try_with(|observed| observed.set(true));
    }
    fail
}

unsafe impl GlobalAlloc for FailOnceAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if should_fail() {
            return std::ptr::null_mut();
        }
        // SAFETY: 동일한 layout으로 시스템 allocator에 위임합니다.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if should_fail() {
            return std::ptr::null_mut();
        }
        // SAFETY: 동일한 layout으로 시스템 allocator에 위임합니다.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: 할당을 위임했던 시스템 allocator에서 같은 포인터와 layout을 해제합니다.
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if should_fail() {
            return std::ptr::null_mut();
        }
        // SAFETY: 기존 포인터와 layout을 시스템 allocator에 전달해 크기를 변경합니다.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}
