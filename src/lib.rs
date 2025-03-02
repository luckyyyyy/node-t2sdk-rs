use encoding::Encoding;
use napi::{bindgen_prelude::Buffer, bindgen_prelude::Object, Result};
use napi_derive::napi;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

mod biz_message;
mod callback;
mod config;
mod connection;
mod def;
mod dylib;
mod f2_packer;
mod f2_result_set;
mod f2_unpacker;
mod iknown;

use biz_message::*;
use config::*;
use connection::*;
use def::*;
use dylib::*;
use f2_packer::*;
use f2_unpacker::*;

static LIBRARY: AtomicPtr<LoadedLibrary> = AtomicPtr::new(ptr::null_mut());
static mut ALLOCATED_LIBRARY: Option<Box<LoadedLibrary>> = None;

fn get_library() -> Result<&'static LoadedLibrary> {
  let ptr = LIBRARY.load(Ordering::Acquire);

  if ptr.is_null() {
    return Err(napi::Error::from_reason("Library not initialized. Call init() first."));
  }

  Ok(unsafe { &*ptr })
}
#[napi]
pub async fn init(lib_path: String) -> Result<()> {
  if !LIBRARY.load(Ordering::Acquire).is_null() {
    return Ok(());
  }

  let loaded_lib = tokio::task::spawn_blocking(move || LoadedLibrary::new(&lib_path))
    .await
    .map_err(|e| napi::Error::from_reason(format!("Task join error: {}", e)))?
    .map_err(|e| napi::Error::from_reason(format!("Failed to load library: {}", e)))?;

  let boxed_lib = Box::new(loaded_lib);
  let lib_ptr = Box::into_raw(boxed_lib);

  match LIBRARY.compare_exchange(ptr::null_mut(), lib_ptr, Ordering::AcqRel, Ordering::Acquire) {
    Ok(_) => {
      unsafe {
        ALLOCATED_LIBRARY = Some(Box::from_raw(lib_ptr));
      }
      Ok(())
    }
    Err(_) => {
      unsafe {
        drop(Box::from_raw(lib_ptr));
      }
      Ok(())
    }
  }
}

#[napi]
pub fn get_version() -> Result<i32> {
  let lib = get_library()?;
  Ok(unsafe { (lib.get_version)() })
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
  Ok(unsafe { (lib.get_pack_version)(ptr) })
}

#[napi]
pub fn new_biz_message() -> Result<BizMessage> {
  BizMessage::new()
}

#[napi(object)]
pub struct T2Header {
  pub function_no: i32,
  pub system_no: Option<i32>,
  pub branch_no: Option<i32>,
  pub sub_system_no: Option<i32>,
  pub company_id: Option<i32>,
}

#[napi]
pub struct T2SDK {
  #[napi(skip)]
  pub config: Config,
  #[napi(skip)]
  pub connection: Option<Connection>,
}

#[napi]
impl T2SDK {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    let config = Config::new()?;
    Ok(Self { config, connection: None })
  }

  #[napi]
  pub fn set_config(&self, config: Object) -> Result<()> {
    let top_keys = Object::keys(&config)?;
    for section in top_keys {
      if let Some(section_obj) = config.get::<_, Object>(&section)? {
        let keys = Object::keys(&section_obj)?;

        for key in keys {
          if let Some(string_val) = section_obj.get::<_, String>(&key)? {
            self
              .config
              .set_string(section.to_string(), key.to_string(), string_val.to_string())?;
          } else if let Some(int_val) = section_obj.get::<_, i32>(&key)? {
            self.config.set_int(section.to_string(), key.to_string(), int_val)?;
          }
        }
      }
    }
    Ok(())
  }

  fn pack_object(&self, obj: &serde_json::Map<String, serde_json::Value>, packer: &Packer, encoding: i8) -> Result<()> {
    for (key, value) in obj {
      if value.is_string() {
        if encoding == 0 {
          let str_val = value.as_str().unwrap();
          let gbk_bytes = encoding::all::GBK.encode(&str_val, encoding::EncoderTrap::Strict).unwrap();
          let buf = Buffer::from(gbk_bytes);
          packer.add_field(key.to_string(), b'R' as i8, buf.len() as i32, 4)?;
        } else {
          let str_val = value.as_str().unwrap();
          packer.add_field(key.to_string(), b'S' as i8, str_val.len() as i32, 4)?;
        }
      } else if value.is_number() {
        if value.is_f64() {
          packer.add_field(key.to_string(), b'D' as i8, 4, 4)?;
        } else {
          packer.add_field(key.to_string(), b'I' as i8, 4, 4)?;
        }
      } else if value.is_array() && value.as_array().unwrap().iter().all(|v| v.is_number()) {
        let arr = value.as_array().unwrap();
        packer.add_field(key.to_string(), b'R' as i8, arr.len() as i32, 4)?;
      }
    }

    for (_, value) in obj {
      if value.is_string() {
        let str_val = value.as_str().unwrap();
        if encoding == 0 {
          let gbk_bytes = encoding::all::GBK.encode(&str_val, encoding::EncoderTrap::Strict).unwrap();
          let buf = Buffer::from(gbk_bytes);
          packer.add_raw(buf)?;
        } else {
          packer.add_str(str_val.to_string())?;
        }
      } else if value.is_number() {
        if value.is_i64() {
          packer.add_int(value.as_i64().unwrap() as i32)?;
        } else if value.is_u64() {
          packer.add_int(value.as_u64().unwrap() as i32)?;
        } else if value.is_f64() {
          packer.add_double(value.as_f64().unwrap() as f64)?;
        }
      } else if value.is_array() && value.as_array().unwrap().iter().all(|v| v.is_number()) {
        let arr = value.as_array().unwrap();
        let bytes: Vec<u8> = arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect();
        let buf: Buffer = Buffer::from(bytes);
        packer.add_raw(buf)?;
      }
    }

    Ok(())
  }

  #[napi]
  pub async fn send(&self, header: T2Header, body: String) -> Result<BizMessage> {
    if !self.is_connected() {
      return Err(napi::Error::from_reason("Not connected".to_string()));
    }

    let json_value: serde_json::Value = match serde_json::from_str(&body) {
      Ok(value) => value,
      Err(_) => return Err(napi::Error::from_reason("Invalid JSON format".to_string())),
    };

    if !json_value.is_object() && !json_value.is_array() {
      return Err(napi::Error::from_reason("JSON must be an object or array".to_string()));
    }

    let biz_message = BizMessage::new()?;
    biz_message.set_packet_type(REQUEST_PACKET)?;
    biz_message.set_function(header.function_no)?;
    if let Some(system_no) = header.system_no {
      biz_message.set_system_no(system_no)?;
    }
    if let Some(branch_no) = header.branch_no {
      biz_message.set_branch_no(branch_no)?;
    }
    if let Some(sub_system_no) = header.sub_system_no {
      biz_message.set_sub_system_no(sub_system_no)?;
    }
    if let Some(company_id) = header.company_id {
      biz_message.set_company_id(company_id)?;
    }
    let packer = Packer::new(PACKER_VERSION_V2)?;
    packer.begin_pack()?;

    // 处理JSON对象
    if json_value.is_object() {
      let obj = json_value.as_object().unwrap();

      // 检查是否为简单包（不包含嵌套对象）
      let is_simple_pack = obj.values().all(|v| !v.is_object() || v.is_null());

      if is_simple_pack {
        self.pack_object(obj, &packer, 0)?;
      } else {
        // 处理复杂包（包含嵌套对象）
        for (key, value) in obj {
          if value.is_object() {
            packer.new_dataset(key.to_string(), 0)?;
            if let Some(nested_obj) = value.as_object() {
              self.pack_object(nested_obj, &packer, 0)?;
            }
          }
        }
      }
    } else if json_value.is_array() {
      // 处理JSON数组
      let arr = json_value.as_array().unwrap();
      for value in arr {
        if let Some(obj) = value.as_object() {
          packer.new_dataset("".to_string(), 0)?;
          self.pack_object(obj, &packer, 0)?;
        }
      }
    }
    packer.end_pack()?;
    let content = packer.get_pack_buf()?;
    biz_message.set_content(content)?;
    let message = biz_message.get_ptr();

    let (tx, rx) = tokio::sync::oneshot::channel::<BizMessage>();

    {
      let conn_guard = self.connection.as_ref();
      let conn = conn_guard.as_ref().unwrap();

      conn.send_biz_msg(message, move |response_msg| {
        let message = BizMessage::new_form_ptr(response_msg);
        let _ = tx.send(message);
      })?;
    }

    match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
      Ok(Ok(message)) => Ok(message),
      Ok(Err(_)) => Err(napi::Error::from_reason("Callback channel closed".to_string())),
      Err(_) => Err(napi::Error::from_reason("Callback timeout".to_string())),
    }
  }

  #[napi]
  pub fn connect(&mut self) -> Result<i32> {
    let ptr = self.config.get_ptr();
    let connection = Connection::new(ptr)?;
    let ret = connection.connect(5000)?;
    if ret != 0 {
      let error_msg = connection.get_error_msg(ret)?;
      return Err(napi::Error::from_reason(error_msg));
    }

    self.connection = Some(connection);
    Ok(ret)
  }

  #[napi(getter)]
  pub fn is_connected(&self) -> bool {
    self.connection.is_some()
  }

  #[napi]
  pub fn disconnect(&mut self) -> Result<()> {
    if let Some(conn) = self.connection.take() {
      conn.close()?;
    }
    Ok(())
  }
}
