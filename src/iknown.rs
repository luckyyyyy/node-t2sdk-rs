use std::ffi::c_void;
use std::os::raw::{c_char, c_ulong};

pub trait IKnown {
  #[allow(dead_code)]
  unsafe fn query_interface(&self, iid: *const c_char, ppv: *mut *mut c_void) -> c_ulong;
  #[allow(dead_code)]
  unsafe fn add_ref(&self) -> c_ulong;
  unsafe fn release(&self) -> c_ulong;
}

#[repr(C)]
pub struct IKnownVTable {
  pub query_interface: unsafe fn(this: *mut c_void, iid: *const c_char, ppv: *mut *mut c_void) -> c_ulong,
  pub add_ref: unsafe fn(this: *mut c_void) -> c_ulong,
  pub release: unsafe fn(this: *mut c_void) -> c_ulong,
}
