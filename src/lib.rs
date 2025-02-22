use libloading::{Library, Symbol};
use napi::Result;
use napi_derive::napi;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

mod config;
mod iknown;
use config::*;

#[derive(Clone)]
struct LoadedLibrary {
    lib: Arc<Library>,
    get_version: Arc<Symbol<'static, unsafe extern "C" fn() -> i32>>,
    new_config: Arc<Symbol<'static, unsafe extern "C" fn() -> *mut CConfigInterfaceRust>>,
}

static LIB: Lazy<Mutex<Option<LoadedLibrary>>> = Lazy::new(|| Mutex::new(None));

fn get_library() -> Result<LoadedLibrary> {
    LIB.lock()
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

    *guard = Some(LoadedLibrary {
        lib,
        get_version,
        new_config,
    });

    Ok(())
}

#[napi]
pub fn get_version() -> Result<i32> {
    let lib = get_library()?;
    Ok(unsafe { (*lib.get_version)() })
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