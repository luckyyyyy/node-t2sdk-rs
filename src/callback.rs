use std::ffi::c_char;
use std::os::raw::{c_int, c_ulong, c_void};

use crate::biz_message::IBizMessageRust;
use crate::biz_message::RouteInfo;
use crate::connection::{CallbackRegistry, IConnectionRust};
use crate::iknown::IKnownVTable;

#[repr(C)]
pub struct RetData {
  pub function_id: i32,
  pub return_code: i32,
  pub error_no: i32,
  pub error_info: *mut i8,
  pub issue_type: i32,
  pub lp_key_info: *mut c_void,
  pub key_info_len: i32,
  pub send_info: RouteInfo,
}

#[repr(C)]
pub(crate) struct CallbackVTable {
  pub iknown: IKnownVTable,
  on_connect: extern "C" fn(this: *mut CCallbackRust, connection: *mut IConnectionRust),
  on_safe_connect: extern "C" fn(this: *mut CCallbackRust, connection: *mut IConnectionRust),
  on_register: extern "C" fn(this: *mut CCallbackRust, connection: *mut IConnectionRust),
  on_close: extern "C" fn(this: *mut CCallbackRust, connection: *mut IConnectionRust),
  on_sent: extern "C" fn(
    this: *mut CCallbackRust,
    connection: *mut IConnectionRust,
    h_send: c_int,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    n_queuing_data: c_int,
  ),
  reserved1: extern "C" fn(this: *mut CCallbackRust, a: *mut c_void, b: *mut c_void, c: *mut c_void, d: *mut c_void),
  reserved2: extern "C" fn(this: *mut CCallbackRust, a: *mut c_void, b: *mut c_void, c: *mut c_void, d: *mut c_void),
  reserved3: extern "C" fn(this: *mut CCallbackRust) -> c_int,
  reserved4: extern "C" fn(this: *mut CCallbackRust),
  reserved5: extern "C" fn(this: *mut CCallbackRust),
  reserved6: extern "C" fn(this: *mut CCallbackRust),
  reserved7: extern "C" fn(this: *mut CCallbackRust),
  on_received_biz: extern "C" fn(
    this: *mut CCallbackRust,
    connection: *mut IConnectionRust,
    h_send: c_int,
    lp_unpacker_or_str: *const c_void,
    n_result: c_int,
  ),
  on_received_biz_ex: extern "C" fn(
    this: *mut CCallbackRust,
    connection: *mut IConnectionRust,
    h_send: c_int,
    lp_ret_data: *mut RetData,
    lp_unpacker_or_str: *const c_void,
    n_result: c_int,
  ),
  on_received_biz_msg:
    extern "C" fn(this: *mut CCallbackRust, connection: *mut IConnectionRust, h_send: c_int, lp_msg: *mut IBizMessageRust),
}

#[repr(C)]
pub struct CCallbackRust {
  vtable: *const CallbackVTable,
  sequence: i32,
}

extern "C" fn callback_query_interface(_this: *mut c_void, _riid: *const c_char, _ppvObject: *mut *mut c_void) -> c_ulong {
  // println!("CCallbackRust::QueryInterface called");
  0
}

extern "C" fn callback_add_ref(_this: *mut c_void) -> c_ulong {
  // println!("CCallbackRust::AddRef called");
  1
}

extern "C" fn callback_release(_this: *mut c_void) -> c_ulong {
  // println!("CCallbackRust::Release called");
  0
}

extern "C" fn on_connect(_this: *mut CCallbackRust, _connection: *mut IConnectionRust) {
  // println!("OnConnect called");
}

extern "C" fn on_safe_connect(_this: *mut CCallbackRust, _connection: *mut IConnectionRust) {
  // println!("OnSafeConnect called");
}

extern "C" fn on_register(_this: *mut CCallbackRust, _connection: *mut IConnectionRust) {
  // println!("OnRegister called");
}

extern "C" fn on_close(_this: *mut CCallbackRust, _connection: *mut IConnectionRust) {
  // println!("OnClose called");
}

extern "C" fn on_sent(
  _this: *mut CCallbackRust,
  _connection: *mut IConnectionRust,
  _h_send: c_int,
  _reserved1: *mut c_void,
  _reserved2: *mut c_void,
  _queuing_data: c_int,
) {
  // println!("OnSent called");
}

extern "C" fn reserved1(_this: *mut CCallbackRust, _a: *mut c_void, _b: *mut c_void, _c: *mut c_void, _d: *mut c_void) {
  // println!("Reserved1 called");
}

extern "C" fn reserved2(_this: *mut CCallbackRust, _a: *mut c_void, _b: *mut c_void, _c: *mut c_void, _d: *mut c_void) {
  // println!("Reserved2 called");
}

extern "C" fn reserved3(_this: *mut CCallbackRust) -> c_int {
  // println!("Reserved3 called");
  0
}

extern "C" fn reserved4(_this: *mut CCallbackRust) {
  // println!("Reserved4 called");
}

extern "C" fn reserved5(_this: *mut CCallbackRust) {
  // println!("Reserved5 called");
}

extern "C" fn reserved6(_this: *mut CCallbackRust) {
  // println!("Reserved6 called");
}

extern "C" fn reserved7(_this: *mut CCallbackRust) {
  // println!("Reserved7 called");
}

extern "C" fn on_received_biz(
  _this: *mut CCallbackRust,
  _connection: *mut IConnectionRust,
  _h_send: c_int,
  _unpacker_or_str: *const c_void,
  _result: c_int,
) {
  // println!("OnReceivedBiz called");
}

extern "C" fn on_received_biz_ex(
  _this: *mut CCallbackRust,
  _connection: *mut IConnectionRust,
  _h_send: c_int,
  _ret_data: *mut RetData,
  _unpacker_or_str: *const c_void,
  _result: c_int,
) {
  // println!("OnReceivedBizEx called");
}

extern "C" fn on_received_biz_msg(this: *mut CCallbackRust, _connection: *mut IConnectionRust, h_send: c_int, msg: *mut IBizMessageRust) {
  unsafe {
    let sequence = (*this).sequence;
    let combined_key = (sequence, h_send);
    let registry = CallbackRegistry::instance();
    registry.invoke_callback(combined_key, msg);
  }
}

pub fn create_callback(sequence: i32) -> Box<CCallbackRust> {
  static VTABLE: CallbackVTable = CallbackVTable {
    iknown: IKnownVTable {
      query_interface: callback_query_interface,
      add_ref: callback_add_ref,
      release: callback_release,
    },
    on_connect,
    on_safe_connect,
    on_register,
    on_close,
    on_sent,
    reserved1,
    reserved2,
    reserved3,
    reserved4,
    reserved5,
    reserved6,
    reserved7,
    on_received_biz,
    on_received_biz_ex,
    on_received_biz_msg,
  };

  Box::new(CCallbackRust {
    vtable: &VTABLE,
    sequence: sequence,
  })
}
