use std::task::{RawWaker, RawWakerVTable};

static VTABLE: RawWakerVTable =
    RawWakerVTable::new(custom_clone, custom_wake, custom_wake_by_ref, custom_drop);

unsafe fn custom_clone(raw_waker: *const ()) -> RawWaker {
    RawWaker::new(raw_waker, &VTABLE)
}

unsafe fn custom_wake(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

unsafe fn custom_wake_by_ref(_raw_waker: *const ()) {}

unsafe fn custom_drop(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

pub fn build_raw_waker() -> RawWaker {
    let data = Box::into_raw(Box::new(88u32));
    RawWaker::new(data as *const (), &VTABLE)
}
