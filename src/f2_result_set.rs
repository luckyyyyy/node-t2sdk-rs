use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

use crate::iknown::{IKnown, IKnownVTable};

pub(crate) trait F2ResultSetInterface: IKnown {
  unsafe fn get_col_count(&self) -> c_int;
  unsafe fn get_col_name(&self, column: c_int) -> *const c_char;
  unsafe fn get_col_type(&self, column: c_int) -> c_char;
  unsafe fn get_col_scale(&self, column: c_int) -> c_int;
  unsafe fn get_col_width(&self, column: c_int) -> c_int;
  unsafe fn find_col_index(&self, column_name: *const c_char) -> c_int;
  unsafe fn get_str_by_index(&self, column: c_int) -> *const c_char;
  unsafe fn get_str(&self, column_name: *const c_char) -> *const c_char;
  unsafe fn get_char_by_index(&self, column: c_int) -> c_char;
  unsafe fn get_char(&self, column_name: *const c_char) -> c_char;
  unsafe fn get_double_by_index(&self, column: c_int) -> f64;
  unsafe fn get_double(&self, column_name: *const c_char) -> f64;
  unsafe fn get_int_by_index(&self, column: c_int) -> c_int;
  unsafe fn get_int(&self, column_name: *const c_char) -> c_int;
  unsafe fn get_raw_by_index(&self, column: c_int, raw_len: *mut c_int) -> *mut c_void;
  unsafe fn get_raw(&self, column_name: *const c_char, raw_len: *mut c_int) -> *mut c_void;
  unsafe fn was_null(&self) -> c_int;
  unsafe fn next(&self);
  unsafe fn is_eof(&self) -> c_int;
  unsafe fn is_empty(&self) -> c_int;
  unsafe fn destroy(&self) -> *mut c_void;
}

#[repr(C)]
pub(crate) struct F2ResultSetVTable {
  pub iknown: IKnownVTable,
  pub get_col_count: extern "C" fn(*mut c_void) -> c_int,
  pub get_col_name: extern "C" fn(*mut c_void, c_int) -> *const c_char,
  pub get_col_type: extern "C" fn(*mut c_void, c_int) -> c_char,
  pub get_col_scale: extern "C" fn(*mut c_void, c_int) -> c_int,
  pub get_col_width: extern "C" fn(*mut c_void, c_int) -> c_int,
  pub find_col_index: extern "C" fn(*mut c_void, *const c_char) -> c_int,
  pub get_str_by_index: extern "C" fn(*mut c_void, c_int) -> *const c_char,
  pub get_str: extern "C" fn(*mut c_void, *const c_char) -> *const c_char,
  pub get_char_by_index: extern "C" fn(*mut c_void, c_int) -> c_char,
  pub get_char: extern "C" fn(*mut c_void, *const c_char) -> c_char,
  pub get_double_by_index: extern "C" fn(*mut c_void, c_int) -> f64,
  pub get_double: extern "C" fn(*mut c_void, *const c_char) -> f64,
  pub get_int_by_index: extern "C" fn(*mut c_void, c_int) -> c_int,
  pub get_int: extern "C" fn(*mut c_void, *const c_char) -> c_int,
  pub get_raw_by_index: extern "C" fn(*mut c_void, c_int, *mut c_int) -> *mut c_void,
  pub get_raw: extern "C" fn(*mut c_void, *const c_char, *mut c_int) -> *mut c_void,
  pub was_null: extern "C" fn(*mut c_void) -> c_int,
  pub next: extern "C" fn(*mut c_void),
  pub is_eof: extern "C" fn(*mut c_void) -> c_int,
  pub is_empty: extern "C" fn(*mut c_void) -> c_int,
  pub destroy: extern "C" fn(*mut c_void) -> *mut c_void,
}
