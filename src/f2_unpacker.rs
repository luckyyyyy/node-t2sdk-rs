use napi::{bindgen_prelude::Buffer, Result};
use napi_derive::napi;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong, c_void};

use crate::f2_result_set::{F2ResultSetInterface, F2ResultSetVTable};
use crate::iknown::IKnown;

pub(crate) trait IF2UnPacker: F2ResultSetInterface {
  unsafe fn get_version(&self) -> c_int;
  unsafe fn open(&self, buffer: *mut c_void, len: c_int) -> c_int;
  unsafe fn get_dataset_count(&self) -> c_int;
  unsafe fn set_current_dataset_by_index(&self, index: c_int) -> c_int;
  unsafe fn set_current_dataset(&self, dataset_name: *const c_char) -> c_int;
  unsafe fn get_pack_buf(&self) -> *mut c_void;
  unsafe fn get_pack_len(&self) -> c_int;
  unsafe fn get_row_count(&self) -> c_int;
  unsafe fn first(&self);
  unsafe fn last(&self);
  unsafe fn go(&self, row: c_int);
  unsafe fn get_dataset_name(&self) -> *const c_char;
  unsafe fn open_and_copy(&self, buffer: *mut c_void, len: c_int) -> c_int;
  unsafe fn get_col_type_by_name(&self, column_name: *const c_char) -> c_char;
  unsafe fn get_col_scale_by_name(&self, column_name: *const c_char) -> c_int;
  unsafe fn get_col_width_by_name(&self, column_name: *const c_char) -> c_int;
}

#[repr(C)]
pub(crate) struct F2UnPackerVTable {
  pub result_set: F2ResultSetVTable,
  pub get_version: unsafe extern "C" fn(*const c_void) -> c_int,
  pub open: unsafe extern "C" fn(*const c_void, *mut c_void, c_int) -> c_int,
  pub get_dataset_count: unsafe extern "C" fn(*const c_void) -> c_int,
  pub set_current_dataset_by_index: unsafe extern "C" fn(*const c_void, c_int) -> c_int,
  pub set_current_dataset: unsafe extern "C" fn(*const c_void, *const c_char) -> c_int,
  pub get_pack_buf: unsafe extern "C" fn(*const c_void) -> *mut c_void,
  pub get_pack_len: unsafe extern "C" fn(*const c_void) -> c_int,
  pub get_row_count: unsafe extern "C" fn(*const c_void) -> c_int,
  pub first: unsafe extern "C" fn(*const c_void),
  pub last: unsafe extern "C" fn(*const c_void),
  pub go: unsafe extern "C" fn(*const c_void, c_int),
  pub get_dataset_name: unsafe extern "C" fn(*const c_void) -> *const c_char,
  pub open_and_copy: unsafe extern "C" fn(*const c_void, *mut c_void, c_int) -> c_int,
  pub get_col_type_by_name: unsafe extern "C" fn(*const c_void, *const c_char) -> c_char,
  pub get_col_scale_by_name: unsafe extern "C" fn(*const c_void, *const c_char) -> c_int,
  pub get_col_width_by_name: unsafe extern "C" fn(*const c_void, *const c_char) -> c_int,
}

#[repr(C)]
pub(crate) struct IF2UnPackerRust {
  vtable: *const F2UnPackerVTable,
}

impl IKnown for IF2UnPackerRust {
  unsafe fn query_interface(&self, iid: *const c_char, ppv: *mut *mut c_void) -> c_ulong {
    ((*self.vtable).result_set.iknown.query_interface)(self as *const _ as *mut c_void, iid, ppv)
  }

  unsafe fn add_ref(&self) -> c_ulong {
    ((*self.vtable).result_set.iknown.add_ref)(self as *const _ as *mut c_void)
  }

  unsafe fn release(&self) -> c_ulong {
    ((*self.vtable).result_set.iknown.release)(self as *const _ as *mut c_void)
  }
}

impl F2ResultSetInterface for IF2UnPackerRust {
  unsafe fn get_col_count(&self) -> c_int {
    ((*self.vtable).result_set.get_col_count)(self as *const _ as *mut c_void)
  }
  unsafe fn get_col_name(&self, column: c_int) -> *const c_char {
    ((*self.vtable).result_set.get_col_name)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_col_type(&self, column: c_int) -> c_char {
    ((*self.vtable).result_set.get_col_type)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_col_scale(&self, column: c_int) -> c_int {
    ((*self.vtable).result_set.get_col_scale)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_col_width(&self, column: c_int) -> c_int {
    ((*self.vtable).result_set.get_col_width)(self as *const _ as *mut c_void, column)
  }
  unsafe fn find_col_index(&self, column_name: *const c_char) -> c_int {
    ((*self.vtable).result_set.find_col_index)(self as *const _ as *mut c_void, column_name)
  }
  unsafe fn get_str_by_index(&self, column: c_int) -> *const c_char {
    ((*self.vtable).result_set.get_str_by_index)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_str(&self, column_name: *const c_char) -> *const c_char {
    ((*self.vtable).result_set.get_str)(self as *const _ as *mut c_void, column_name)
  }
  unsafe fn get_char_by_index(&self, column: c_int) -> c_char {
    ((*self.vtable).result_set.get_char_by_index)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_char(&self, column_name: *const c_char) -> c_char {
    ((*self.vtable).result_set.get_char)(self as *const _ as *mut c_void, column_name)
  }
  unsafe fn get_double_by_index(&self, column: c_int) -> f64 {
    ((*self.vtable).result_set.get_double_by_index)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_double(&self, column_name: *const c_char) -> f64 {
    ((*self.vtable).result_set.get_double)(self as *const _ as *mut c_void, column_name)
  }
  unsafe fn get_int_by_index(&self, column: c_int) -> c_int {
    ((*self.vtable).result_set.get_int_by_index)(self as *const _ as *mut c_void, column)
  }
  unsafe fn get_int(&self, column_name: *const c_char) -> c_int {
    ((*self.vtable).result_set.get_int)(self as *const _ as *mut c_void, column_name)
  }
  unsafe fn get_raw_by_index(&self, column: c_int, raw_len: *mut c_int) -> *mut c_void {
    ((*self.vtable).result_set.get_raw_by_index)(self as *const _ as *mut c_void, column, raw_len)
  }
  unsafe fn get_raw(&self, column_name: *const c_char, raw_len: *mut c_int) -> *mut c_void {
    ((*self.vtable).result_set.get_raw)(self as *const _ as *mut c_void, column_name, raw_len)
  }
  unsafe fn was_null(&self) -> c_int {
    ((*self.vtable).result_set.was_null)(self as *const _ as *mut c_void)
  }
  unsafe fn next(&self) {
    ((*self.vtable).result_set.next)(self as *const _ as *mut c_void)
  }
  unsafe fn is_eof(&self) -> c_int {
    ((*self.vtable).result_set.is_eof)(self as *const _ as *mut c_void)
  }
  unsafe fn is_empty(&self) -> c_int {
    ((*self.vtable).result_set.is_empty)(self as *const _ as *mut c_void)
  }
  unsafe fn destroy(&self) -> *mut c_void {
    ((*self.vtable).result_set.destroy)(self as *const _ as *mut c_void)
  }
}

impl IF2UnPacker for IF2UnPackerRust {
  unsafe fn get_version(&self) -> c_int {
    ((*self.vtable).get_version)(self as *const _ as *mut c_void)
  }

  unsafe fn open(&self, buffer: *mut c_void, len: c_int) -> c_int {
    ((*self.vtable).open)(self as *const _ as *mut c_void, buffer, len)
  }

  unsafe fn get_dataset_count(&self) -> c_int {
    ((*self.vtable).get_dataset_count)(self as *const _ as *mut c_void)
  }

  unsafe fn set_current_dataset_by_index(&self, index: c_int) -> c_int {
    ((*self.vtable).set_current_dataset_by_index)(self as *const _ as *mut c_void, index)
  }

  unsafe fn set_current_dataset(&self, dataset_name: *const c_char) -> c_int {
    ((*self.vtable).set_current_dataset)(self as *const _ as *mut c_void, dataset_name)
  }

  unsafe fn get_pack_buf(&self) -> *mut c_void {
    ((*self.vtable).get_pack_buf)(self as *const _ as *mut c_void)
  }

  unsafe fn get_pack_len(&self) -> c_int {
    ((*self.vtable).get_pack_len)(self as *const _ as *mut c_void)
  }

  unsafe fn get_row_count(&self) -> c_int {
    ((*self.vtable).get_row_count)(self as *const _ as *mut c_void)
  }

  unsafe fn first(&self) {
    ((*self.vtable).first)(self as *const _ as *mut c_void)
  }

  unsafe fn last(&self) {
    ((*self.vtable).last)(self as *const _ as *mut c_void)
  }

  unsafe fn go(&self, row: c_int) {
    ((*self.vtable).go)(self as *const _ as *mut c_void, row)
  }

  unsafe fn get_dataset_name(&self) -> *const c_char {
    ((*self.vtable).get_dataset_name)(self as *const _ as *mut c_void)
  }

  unsafe fn open_and_copy(&self, buffer: *mut c_void, len: c_int) -> c_int {
    ((*self.vtable).open_and_copy)(self as *const _ as *mut c_void, buffer, len)
  }

  unsafe fn get_col_type_by_name(&self, column_name: *const c_char) -> c_char {
    ((*self.vtable).get_col_type_by_name)(self as *const _ as *mut c_void, column_name)
  }

  unsafe fn get_col_scale_by_name(&self, column_name: *const c_char) -> c_int {
    ((*self.vtable).get_col_scale_by_name)(self as *const _ as *mut c_void, column_name)
  }
  unsafe fn get_col_width_by_name(&self, column_name: *const c_char) -> c_int {
    ((*self.vtable).get_col_width_by_name)(self as *const _ as *mut c_void, column_name)
  }
}

#[napi]
pub struct UnPacker {
  unpacker_ptr: *mut IF2UnPackerRust,
}

impl UnPacker {
  pub fn new(ptr: *mut c_void, len: c_int, version: i32) -> Result<UnPacker> {
    let lib = crate::get_library()?;

    let unpacker_ptr = unsafe {
      if version == 0x20 {
        (lib.new_unpacker_v1)(ptr, len)
      } else {
        (lib.new_unpacker)(ptr, len)
      }
    };

    if unpacker_ptr.is_null() {
      return Err(napi::Error::from_reason("Failed to create unpacker instance"));
    }

    Ok(UnPacker { unpacker_ptr })
  }

  pub(crate) fn from_ptr(ptr: *mut IF2UnPackerRust) -> Self {
    UnPacker { unpacker_ptr: ptr }
  }

  fn check_ptr(&self) -> Result<&IF2UnPackerRust> {
    unsafe {
      if self.unpacker_ptr.is_null() {
        return Err(napi::Error::from_reason("UnPacker pointer is null"));
      }
      Ok(&*self.unpacker_ptr)
    }
  }

  fn to_c_string(s: String, field: &str) -> Result<CString> {
    CString::new(s).map_err(|e| napi::Error::from_reason(format!("Invalid {}: {}", field, e)))
  }
}

#[napi]
impl UnPacker {
  #[napi]

  pub fn get_col_count(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_col_count() })
  }

  #[napi]
  pub fn get_col_name(&self, column: i32) -> Result<String> {
    let rs = self.check_ptr()?;
    let col_name = unsafe { CStr::from_ptr(rs.get_col_name(column)).to_string_lossy().into_owned() };
    Ok(col_name)
  }

  #[napi]
  pub fn get_col_type(&self, column: i32) -> Result<i8> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_col_type(column) })
  }

  #[napi]
  pub fn get_col_scale(&self, column: i32) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_col_scale(column) })
  }

  #[napi]
  pub fn get_col_width(&self, column: i32) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_col_width(column) })
  }

  #[napi]
  pub fn find_col_index(&self, column_name: String) -> Result<i32> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.find_col_index(column_name.as_ptr()) })
  }

  #[napi]
  pub fn get_str_by_index(&self, column: i32) -> Result<String> {
    let rs = self.check_ptr()?;
    let str_val = unsafe { CStr::from_ptr(rs.get_str_by_index(column)).to_string_lossy().into_owned() };
    Ok(str_val)
  }

  #[napi]
  pub fn get_str(&self, column_name: String) -> Result<String> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    let str_val = unsafe { CStr::from_ptr(rs.get_str(column_name.as_ptr())).to_string_lossy().into_owned() };
    Ok(str_val)
  }

  #[napi]
  pub fn get_char_by_index(&self, column: i32) -> Result<i8> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_char_by_index(column) })
  }

  #[napi]
  pub fn get_char(&self, column_name: String) -> Result<i8> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.get_char(column_name.as_ptr()) })
  }

  #[napi]
  pub fn get_double_by_index(&self, column: i32) -> Result<f64> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_double_by_index(column) })
  }

  #[napi]
  pub fn get_double(&self, column_name: String) -> Result<f64> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.get_double(column_name.as_ptr()) })
  }

  #[napi]
  pub fn get_int_by_index(&self, column: i32) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_int_by_index(column) })
  }

  #[napi]
  pub fn get_int(&self, column_name: String) -> Result<i32> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.get_int(column_name.as_ptr()) })
  }
  #[napi]
  pub fn get_raw_by_index(&self, column: i32) -> Result<Buffer> {
    let rs = self.check_ptr()?;
    let mut raw_len = 0;
    let ptr = unsafe { rs.get_raw_by_index(column, &mut raw_len) };
    let buffer = unsafe { Buffer::from(std::slice::from_raw_parts(ptr as *const u8, raw_len as usize)) };
    Ok(buffer)
  }
  #[napi]
  pub fn get_raw(&self, column_name: String) -> Result<Buffer> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    let mut raw_len = 0;
    unsafe {
      let ptr = rs.get_raw(column_name.as_ptr(), &mut raw_len);
      let buffer = Buffer::from(std::slice::from_raw_parts(ptr as *const u8, raw_len as usize));
      Ok(buffer)
    }
  }
  #[napi]
  pub fn was_null(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.was_null() })
  }
  #[napi]
  pub fn next(&self) -> Result<()> {
    let rs = self.check_ptr()?;
    unsafe {
      rs.next();
    }
    Ok(())
  }

  #[napi]
  pub fn is_eof(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.is_eof() })
  }

  #[napi]
  pub fn is_empty(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.is_empty() })
  }
  #[napi]
  pub fn destroy(&self) -> Result<()> {
    let rs = self.check_ptr()?;
    unsafe {
      rs.destroy();
    }
    Ok(())
  }
  #[napi]
  pub fn get_version(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_version() })
  }
  #[napi]
  pub fn open(&self, buffer: Buffer) -> Result<i32> {
    let rs = self.check_ptr()?;
    let buffer = buffer.as_ref();
    let len = buffer.len() as i32;
    let ptr = buffer.as_ptr() as *mut c_void;
    Ok(unsafe { rs.open(ptr, len) })
  }
  #[napi]
  pub fn get_dataset_count(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_dataset_count() })
  }
  #[napi]
  pub fn set_current_dataset_by_index(&self, index: i32) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.set_current_dataset_by_index(index) })
  }
  #[napi]
  pub fn set_current_dataset(&self, dataset_name: String) -> Result<i32> {
    let rs = self.check_ptr()?;
    let dataset_name = UnPacker::to_c_string(dataset_name, "dataset_name")?;
    Ok(unsafe { rs.set_current_dataset(dataset_name.as_ptr()) })
  }

  #[napi]
  pub unsafe fn get_pack_buf(&self) -> Result<Buffer> {
    let rs = self.check_ptr()?;
    let ptr = rs.get_pack_buf();
    let len = rs.get_pack_len();
    let buffer = Buffer::from(std::slice::from_raw_parts(ptr as *const u8, len as usize));
    Ok(buffer)
  }
  #[napi]
  pub fn get_row_count(&self) -> Result<i32> {
    let rs = self.check_ptr()?;
    Ok(unsafe { rs.get_row_count() })
  }
  #[napi]
  pub fn first(&self) -> Result<()> {
    let rs = self.check_ptr()?;
    unsafe {
      rs.first();
    }
    Ok(())
  }
  #[napi]
  pub fn last(&self) -> Result<()> {
    let rs = self.check_ptr()?;
    unsafe {
      rs.last();
    }
    Ok(())
  }
  #[napi]
  pub fn go(&self, row: i32) -> Result<()> {
    let rs = self.check_ptr()?;
    unsafe {
      rs.go(row);
    }
    Ok(())
  }
  #[napi]
  pub fn get_dataset_name(&self) -> Result<String> {
    let rs = self.check_ptr()?;
    let dataset_name = unsafe { CStr::from_ptr(rs.get_dataset_name()).to_string_lossy().into_owned() };
    Ok(dataset_name)
  }
  #[napi]
  pub fn open_and_copy(&self, buffer: Buffer) -> Result<i32> {
    let rs = self.check_ptr()?;
    let buffer = buffer.as_ref();
    let len = buffer.len() as i32;
    let ptr = buffer.as_ptr() as *mut c_void;
    Ok(unsafe { rs.open_and_copy(ptr, len) })
  }
  #[napi]
  pub fn get_col_type_by_name(&self, column_name: String) -> Result<i8> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.get_col_type_by_name(column_name.as_ptr()) })
  }
  #[napi]
  pub fn get_col_scale_by_name(&self, column_name: String) -> Result<i32> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.get_col_scale_by_name(column_name.as_ptr()) })
  }
  #[napi]
  pub fn get_col_width_by_name(&self, column_name: String) -> Result<i32> {
    let rs = self.check_ptr()?;
    let column_name = UnPacker::to_c_string(column_name, "column_name")?;
    Ok(unsafe { rs.get_col_width_by_name(column_name.as_ptr()) })
  }
}

impl Drop for UnPacker {
  fn drop(&mut self) {
    unsafe {
      if !self.unpacker_ptr.is_null() {
        let ptr = &*self.unpacker_ptr;
        ptr.release();
        self.unpacker_ptr = std::ptr::null_mut();
      }
    }
  }
}
