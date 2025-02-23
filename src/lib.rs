use libloading::{Library, Symbol};
use napi::{bindgen_prelude::Buffer, Result};
use napi_derive::napi;
use once_cell::sync::Lazy;
use std::os::raw::{c_int, c_void};
use std::sync::{Arc, Mutex};

mod biz_message;
mod config;
mod def;
mod f2_packer;
mod f2_result_set;
mod f2_unpacker;
mod iknown;
// mod if2_unpacker;
use biz_message::*;
use config::*;
use f2_packer::*;
use f2_unpacker::*;

#[derive(Clone)]
struct LoadedLibrary {
  #[allow(dead_code)]
  lib: Arc<Library>,
  get_version: Arc<Symbol<'static, unsafe extern "C" fn() -> i32>>,
  new_config: Arc<Symbol<'static, unsafe extern "C" fn() -> *mut CConfigInterfaceRust>>,
  new_packer: Arc<Symbol<'static, unsafe extern "C" fn(i32) -> *mut IF2PackerRust>>,
  new_unpacker: Arc<Symbol<'static, unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust>>,
  new_unpacker_v1: Arc<Symbol<'static, unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust>>,
  get_pack_version: Arc<Symbol<'static, unsafe extern "C" fn(*mut c_void) -> i32>>,
  new_biz_message: Arc<Symbol<'static, unsafe extern "C" fn() -> *mut IBizMessageRust>>,
}

static LIB: Lazy<Mutex<Option<LoadedLibrary>>> = Lazy::new(|| Mutex::new(None));

fn get_library() -> Result<LoadedLibrary> {
  LIB
    .lock()
    .unwrap()
    .as_ref()
    .ok_or_else(|| napi::Error::from_reason("Library not initialized. Call init() first."))
    .map(Clone::clone)
}

#[napi]
pub async fn init(lib_path: String) -> Result<()> {
  let mut guard = LIB.lock().unwrap();
  if guard.is_some() {
    return Err(napi::Error::from_reason("Library already initialized"));
  }

  let lib = unsafe { Library::new(&lib_path).map_err(|e| napi::Error::from_reason(format!("Failed to load library: {}", e)))? };
  let lib = Arc::new(lib);

  let get_version = unsafe {
    let symbol: Symbol<unsafe extern "C" fn() -> i32> = lib
      .get(b"GetVersionInfo")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get version symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };

  let new_config = unsafe {
    let symbol: Symbol<unsafe extern "C" fn() -> *mut CConfigInterfaceRust> = lib
      .get(b"NewConfig")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get NewConfig symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };

  let new_packer = unsafe {
    let symbol: Symbol<unsafe extern "C" fn(i32) -> *mut IF2PackerRust> = lib
      .get(b"NewPacker")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get NewPacker symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };

  let new_unpacker = unsafe {
    let symbol: Symbol<unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust> = lib
      .get(b"NewUnPacker")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get NewUnPacker symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };
  // NewUnPackerV1
  let new_unpacker_v1 = unsafe {
    let symbol: Symbol<unsafe extern "C" fn(*mut c_void, c_int) -> *mut IF2UnPackerRust> = lib
      .get(b"NewUnPackerV1")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get NewUnPackerV1 symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };
  let get_pack_version = unsafe {
    let symbol: Symbol<unsafe extern "C" fn() -> i32> = lib
      .get(b"GetPackVersion")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get GetPackVersion symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };

  let new_biz_message = unsafe {
    let symbol: Symbol<unsafe extern "C" fn() -> *mut IBizMessageRust> = lib
      .get(b"NewBizMessage")
      .map_err(|e| napi::Error::from_reason(format!("Failed to get NewBizMessage symbol: {}", e)))?;
    Arc::new(std::mem::transmute(symbol))
  };

  *guard = Some(LoadedLibrary {
    lib,
    get_version,
    new_config,
    new_packer,
    new_unpacker,
    new_unpacker_v1,
    get_pack_version,
    new_biz_message,
  });

  Ok(())
}

#[napi]
pub fn get_version() -> Result<i32> {
  let lib = get_library()?;
  Ok(unsafe { (*lib.get_version)() })
}

#[napi]
pub fn new_packer(version: i32) -> Result<Packer> {
  Packer::new(version)
}

#[napi]
pub fn new_unpacker(buffer: Buffer) -> Result<UnPacker> {
  let len = buffer.len() as c_int;
  let ptr = buffer.as_ptr() as *mut c_void;
  UnPacker::new(ptr, len, 0x20)
}

#[napi]
pub fn new_unpacker_v1(buffer: Buffer) -> Result<UnPacker> {
  let len = buffer.len() as c_int;
  let ptr = buffer.as_ptr() as *mut c_void;
  UnPacker::new(ptr, len, 1)
}

#[napi]
pub fn get_pack_version(buffer: Buffer) -> Result<i32> {
  // let len = buffer.len() as c_int;
  let ptr = buffer.as_ptr() as *mut c_void;
  let lib = get_library()?;
  Ok(unsafe { (*lib.get_pack_version)(ptr) })
}

#[napi]
pub fn new_biz_message() -> Result<BizMessage> {
  BizMessage::new()
}

#[napi]
pub struct T2SDK {
  #[napi(readonly)]
  pub config: Config,
}

#[napi]
impl T2SDK {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    let config = Config::new()?;
    Ok(Self { config })
  }
}
