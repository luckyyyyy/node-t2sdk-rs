use napi::Result;
use napi_derive::napi;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong};

use crate::iknown::{IKnown, IKnownVTable};

pub(crate) trait CConfigInterface: IKnown {
  unsafe fn load(&self, sz_file_name: *const c_char) -> c_int;
  unsafe fn save(&self, sz_file_name: *const c_char) -> c_int;
  unsafe fn get_string(&self, sz_section: *const c_char, sz_entry: *const c_char, sz_default: *const c_char) -> *const c_char;
  unsafe fn get_int(&self, sz_section: *const c_char, sz_entry: *const c_char, i_default: c_int) -> c_int;
  unsafe fn set_string(&self, sz_section: *const c_char, sz_entry: *const c_char, sz_value: *const c_char) -> c_int;
  unsafe fn set_int(&self, sz_section: *const c_char, sz_entry: *const c_char, i_value: c_int) -> c_int;
}

#[repr(C)]
pub(crate) struct VTable {
  pub iknown: IKnownVTable,
  load: LoadFn,
  save: SaveFn,
  get_string: GetStringFn,
  get_int: GetIntFn,
  set_string: SetStringFn,
  set_int: SetIntFn,
}

type LoadFn = unsafe fn(this: *mut c_void, sz_file_name: *const c_char) -> c_int;
type SaveFn = unsafe fn(this: *mut c_void, sz_file_name: *const c_char) -> c_int;
type GetStringFn =
  unsafe fn(this: *mut c_void, sz_section: *const c_char, sz_entry: *const c_char, sz_default: *const c_char) -> *const c_char;
type GetIntFn = unsafe fn(this: *mut c_void, sz_section: *const c_char, sz_entry: *const c_char, i_default: c_int) -> c_int;
type SetStringFn = unsafe fn(this: *mut c_void, sz_section: *const c_char, sz_entry: *const c_char, sz_value: *const c_char) -> c_int;
type SetIntFn = unsafe fn(this: *mut c_void, sz_section: *const c_char, sz_entry: *const c_char, i_value: c_int) -> c_int;

#[repr(C)]
pub(crate) struct CConfigInterfaceRust {
  vtable: *const VTable,
}

impl IKnown for CConfigInterfaceRust {
  unsafe fn query_interface(&self, iid: *const c_char, ppv: *mut *mut c_void) -> c_ulong {
    ((*self.vtable).iknown.query_interface)(self as *const _ as *mut c_void, iid, ppv)
  }

  unsafe fn add_ref(&self) -> c_ulong {
    ((*self.vtable).iknown.add_ref)(self as *const _ as *mut c_void)
  }

  unsafe fn release(&self) -> c_ulong {
    ((*self.vtable).iknown.release)(self as *const _ as *mut c_void)
  }
}

impl CConfigInterface for CConfigInterfaceRust {
  unsafe fn load(&self, sz_file_name: *const c_char) -> c_int {
    ((*self.vtable).load)(self as *const _ as *mut c_void, sz_file_name)
  }

  unsafe fn save(&self, sz_file_name: *const c_char) -> c_int {
    ((*self.vtable).save)(self as *const _ as *mut c_void, sz_file_name)
  }

  unsafe fn get_string(&self, sz_section: *const c_char, sz_entry: *const c_char, sz_default: *const c_char) -> *const c_char {
    ((*self.vtable).get_string)(self as *const _ as *mut c_void, sz_section, sz_entry, sz_default)
  }

  unsafe fn get_int(&self, sz_section: *const c_char, sz_entry: *const c_char, i_default: c_int) -> c_int {
    ((*self.vtable).get_int)(self as *const _ as *mut c_void, sz_section, sz_entry, i_default)
  }

  unsafe fn set_string(&self, sz_section: *const c_char, sz_entry: *const c_char, sz_value: *const c_char) -> c_int {
    ((*self.vtable).set_string)(self as *const _ as *mut c_void, sz_section, sz_entry, sz_value)
  }

  unsafe fn set_int(&self, sz_section: *const c_char, sz_entry: *const c_char, i_value: c_int) -> c_int {
    ((*self.vtable).set_int)(self as *const _ as *mut c_void, sz_section, sz_entry, i_value)
  }
}

#[napi]
#[derive(Clone)]
pub struct Config {
  config_ptr: *mut CConfigInterfaceRust,
}

impl Config {
  pub fn new() -> Result<Self> {
    let lib = crate::get_library()?;

    unsafe {
      let config_ptr = (*lib.new_config)();
      if config_ptr.is_null() {
        return Err(napi::Error::from_reason("Failed to create config instance"));
      }

      Ok(Config { config_ptr })
    }
  }

  fn check_ptr(&self) -> Result<&CConfigInterfaceRust> {
    unsafe {
      if self.config_ptr.is_null() {
        return Err(napi::Error::from_reason("Config pointer is null"));
      }
      Ok(&*self.config_ptr)
    }
  }

  fn to_c_string(s: String, field: &str) -> Result<CString> {
    CString::new(s).map_err(|e| napi::Error::from_reason(format!("Invalid {}: {}", field, e)))
  }
}

#[napi]
impl Config {
  #[napi]
  pub fn load(&self, file_name: String) -> Result<i32> {
    let config = self.check_ptr()?;
    let c_file_name = Self::to_c_string(file_name, "filename")?;

    unsafe { Ok(config.load(c_file_name.as_ptr())) }
  }

  #[napi]
  pub fn get_string(&self, section: String, entry: String, default: String) -> Result<String> {
    let config = self.check_ptr()?;
    let c_section = Self::to_c_string(section, "section")?;
    let c_entry = Self::to_c_string(entry, "entry")?;
    let c_default = Self::to_c_string(default, "default")?;

    unsafe {
      let value_ptr = config.get_string(c_section.as_ptr(), c_entry.as_ptr(), c_default.as_ptr());

      if value_ptr.is_null() {
        return Err(napi::Error::from_reason("Null string returned from get_string"));
      }

      CStr::from_ptr(value_ptr)
        .to_str()
        .map_err(|e| napi::Error::from_reason(format!("Invalid string encoding: {}", e)))
        .map(String::from)
    }
  }

  #[napi]
  pub fn get_int(&self, section: String, entry: String, default: i32) -> Result<i32> {
    let config = self.check_ptr()?;
    let c_section = Self::to_c_string(section, "section")?;
    let c_entry = Self::to_c_string(entry, "entry")?;

    unsafe { Ok(config.get_int(c_section.as_ptr(), c_entry.as_ptr(), default)) }
  }

  #[napi]
  pub fn set_string(&self, section: String, entry: String, value: String) -> Result<i32> {
    let config = self.check_ptr()?;
    let c_section = Self::to_c_string(section, "section")?;
    let c_entry = Self::to_c_string(entry, "entry")?;
    let c_value = Self::to_c_string(value, "value")?;

    unsafe { Ok(config.set_string(c_section.as_ptr(), c_entry.as_ptr(), c_value.as_ptr())) }
  }

  #[napi]
  pub fn set_int(&self, section: String, entry: String, value: i32) -> Result<i32> {
    let config = self.check_ptr()?;
    let c_section = Self::to_c_string(section, "section")?;
    let c_entry = Self::to_c_string(entry, "entry")?;

    unsafe { Ok(config.set_int(c_section.as_ptr(), c_entry.as_ptr(), value)) }
  }

  #[napi]
  pub fn save(&self, file_name: String) -> Result<i32> {
    let config = self.check_ptr()?;
    let c_file_name = Self::to_c_string(file_name, "filename")?;

    unsafe { Ok(config.save(c_file_name.as_ptr())) }
  }
}

impl Drop for Config {
  fn drop(&mut self) {
    unsafe {
      if !self.config_ptr.is_null() {
        let config = &*self.config_ptr;
        config.release();
        self.config_ptr = std::ptr::null_mut();
      }
    }
  }
}
