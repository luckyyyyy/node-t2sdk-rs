use std::ffi::CString;
use std::path::Path;

use crate::biz_message::IBizMessageRust;
use crate::config::CConfigInterfaceRust;
use crate::connection::IConnectionRust;
use crate::f2_packer::IF2PackerRust;
use crate::f2_unpacker::IF2UnPackerRust;
use std::os::raw::{c_int, c_void};

#[cfg(unix)]
use libc::{dlopen, dlsym, RTLD_DEEPBIND, RTLD_LAZY, RTLD_LOCAL};
#[cfg(windows)]
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA, HMODULE};

pub struct DynamicLibrary {
  #[cfg(unix)]
  handle: *mut libc::c_void,
  #[cfg(windows)]
  handle: HMODULE,
}

impl DynamicLibrary {
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
    let path_str = path.as_ref().to_str().ok_or("Invalid path")?.to_string();

    #[cfg(unix)]
    {
      let path_cstring = CString::new(path_str).map_err(|e| e.to_string())?;
      unsafe {
        #[cfg(target_os = "macos")]
        let handle = dlopen(path_cstring.as_ptr(), RTLD_LAZY | RTLD_LOCAL);

        #[cfg(target_os = "linux")]
        let handle = dlopen(path_cstring.as_ptr(), RTLD_LAZY | RTLD_LOCAL | RTLD_DEEPBIND);

        if handle.is_null() {
          let error = std::ffi::CStr::from_ptr(libc::dlerror()).to_string_lossy().into_owned();
          return Err(format!("Failed to load library: {}", error));
        }

        Ok(DynamicLibrary { handle })
      }
    }

    #[cfg(windows)]
    {
      let path_cstring = CString::new(path_str).map_err(|e| e.to_string())?;
      unsafe {
        let handle = LoadLibraryA(path_cstring.as_ptr());
        if handle.is_null() {
          return Err("Failed to load library".to_string());
        }
        Ok(DynamicLibrary { handle })
      }
    }
  }

  pub unsafe fn get_symbol<T>(&self, symbol: &str) -> Result<T, String> {
    #[cfg(unix)]
    {
      let symbol = CString::new(symbol).map_err(|e| e.to_string())?;
      let symbol_ptr = dlsym(self.handle, symbol.as_ptr());
      if symbol_ptr.is_null() {
        let error = std::ffi::CStr::from_ptr(libc::dlerror()).to_string_lossy().into_owned();
        return Err(format!("Failed to get symbol: {}", error));
      }
      Ok(std::mem::transmute_copy(&symbol_ptr))
    }

    #[cfg(windows)]
    {
      let symbol = CString::new(symbol).map_err(|e| e.to_string())?;
      let symbol_ptr = GetProcAddress(self.handle, symbol.as_ptr());
      if symbol_ptr.is_null() {
        return Err("Failed to get symbol".to_string());
      }
      Ok(std::mem::transmute_copy(&symbol_ptr))
    }
  }
}

impl Drop for DynamicLibrary {
  fn drop(&mut self) {
    unsafe {
      #[cfg(unix)]
      libc::dlclose(self.handle);

      #[cfg(windows)]
      winapi::um::libloaderapi::FreeLibrary(self.handle);
    }
  }
}

unsafe impl Send for DynamicLibrary {}
unsafe impl Sync for DynamicLibrary {}

pub(crate) struct LoadedLibrary {
  #[allow(dead_code)]
  lib: DynamicLibrary,
  pub get_version: unsafe extern "C" fn() -> i32,
  pub new_config: unsafe extern "C" fn() -> *mut CConfigInterfaceRust,
  pub new_packer: unsafe extern "C" fn(i32) -> *mut IF2PackerRust,
  pub new_unpacker: unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust,
  pub new_unpacker_v1: unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust,
  pub get_pack_version: unsafe extern "C" fn(*mut c_void) -> i32,
  pub new_biz_message: unsafe extern "C" fn() -> *mut IBizMessageRust,
  pub new_connection: unsafe extern "C" fn(*mut CConfigInterfaceRust) -> *mut IConnectionRust,
}

impl LoadedLibrary {
  pub fn new(path: &str) -> Result<Self, String> {
    let lib = DynamicLibrary::new(path)?;

    unsafe {
      let get_version: unsafe extern "C" fn() -> i32 = lib.get_symbol("GetVersionInfo")?;
      let new_config: unsafe extern "C" fn() -> *mut CConfigInterfaceRust = lib.get_symbol("NewConfig")?;
      let new_packer: unsafe extern "C" fn(i32) -> *mut IF2PackerRust = lib.get_symbol("NewPacker")?;
      let new_unpacker: unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust = lib.get_symbol("NewUnPacker")?;
      let new_unpacker_v1: unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust = lib.get_symbol("NewUnPackerV1")?;
      let get_pack_version: unsafe extern "C" fn(*mut c_void) -> i32 = lib.get_symbol("GetPackVersion")?;
      let new_biz_message: unsafe extern "C" fn() -> *mut IBizMessageRust = lib.get_symbol("NewBizMessage")?;
      let new_connection: unsafe extern "C" fn(*mut CConfigInterfaceRust) -> *mut IConnectionRust = lib.get_symbol("NewConnection")?;

      Ok(LoadedLibrary {
        lib,
        get_version,
        new_config,
        new_packer,
        new_unpacker,
        new_unpacker_v1,
        get_pack_version,
        new_biz_message,
        new_connection,
      })
    }
  }
}
