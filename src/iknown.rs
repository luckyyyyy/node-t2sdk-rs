use std::ffi::c_void;
use std::os::raw::{c_char, c_ulong};

pub trait IKnown {
  unsafe fn query_interface(&self, iid: *const c_char, ppv: *mut *mut c_void) -> c_ulong;
  unsafe fn add_ref(&self) -> c_ulong;
  unsafe fn release(&self) -> c_ulong;
}

pub type QueryInterfaceFn = unsafe fn(this: *mut c_void, iid: *const c_char, ppv: *mut *mut c_void) -> c_ulong;
pub type AddRefFn = unsafe fn(this: *mut c_void) -> c_ulong;
pub type ReleaseFn = unsafe fn(this: *mut c_void) -> c_ulong;

#[repr(C)]
pub struct IKnownVTable {
  pub query_interface: QueryInterfaceFn,
  pub add_ref: AddRefFn,
  pub release: ReleaseFn,
}
