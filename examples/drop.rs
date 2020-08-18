#![feature(ptr_internals, allocator_api)]

use std::alloc::{AllocRef, Global, GlobalAlloc, Layout};
use std::mem;
use std::ptr::{drop_in_place, NonNull, Unique};

struct MyBox<T>{ ptr: Unique<T> }

impl<T> Drop for MyBox<T> {
  fn drop(&mut self) {
    unsafe {
      drop_in_place(self.ptr.as_ptr());
      let c: NonNull<T> = self.ptr.into();
      Global.dealloc(c.cast(), Layout::new::<T>())
    }
  }
}

fn main() {
  let mut i = 3;
  let a = MyBox { ptr: Unique::new(&mut i).unwrap() };
}
