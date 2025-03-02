extern crate encoding;
use encoding::Encoding;
use napi::{bindgen_prelude::Buffer, Error, Result, Status};
use napi_derive::napi;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong};

// use crate::f2_packer::Packer;
use crate::iknown::{IKnown, IKnownVTable};

pub const IDENTITY_NAME_LENGTH: usize = 32;
pub const ID_LENGTH: usize = 4;
pub const ID_STR_LEN: usize = IDENTITY_NAME_LENGTH + ID_LENGTH + 1;

pub const PLUGINID_LENGTH: usize = 256;
pub const PLUGIN_NAME_LENGTH: usize = PLUGINID_LENGTH + ID_LENGTH + 1;
pub const SVR_NAME_LENGTH: usize = 256;
pub const SVRINSTANCE_NAME_LENGTH: usize = SVR_NAME_LENGTH + ID_LENGTH + 1;

#[repr(C)]
pub struct RouteInfo {
  pub ospf_name: [c_char; ID_STR_LEN + 1],
  pub nbr_name: [c_char; ID_STR_LEN + 1],
  pub svr_name: [c_char; SVRINSTANCE_NAME_LENGTH + 1],
  pub plugin_id: [c_char; PLUGIN_NAME_LENGTH + 1],
  pub connect_id: c_int,
  pub member_no: c_int,
}

impl Default for RouteInfo {
  /// 模拟C++的memset(this,0,sizeof(tagRouteInfo))
  fn default() -> Self {
    Self {
      ospf_name: [0; ID_STR_LEN + 1],
      nbr_name: [0; ID_STR_LEN + 1],
      svr_name: [0; SVRINSTANCE_NAME_LENGTH + 1],
      plugin_id: [0; PLUGIN_NAME_LENGTH + 1],
      connect_id: 0,
      member_no: 0,
    }
  }
}

pub(crate) trait IBizMessage: IKnown {
  unsafe fn set_function(&self, function_no: c_int);
  unsafe fn get_function(&self) -> c_int;

  unsafe fn set_packet_type(&self, packet_type: c_int);
  unsafe fn get_packet_type(&self) -> c_int;

  unsafe fn set_branch_no(&self, branch_no: c_int);
  unsafe fn get_branch_no(&self) -> c_int;

  unsafe fn set_system_no(&self, system_no: c_int);
  unsafe fn get_system_no(&self) -> c_int;

  unsafe fn set_sub_system_no(&self, sub_system_no: c_int);
  unsafe fn get_sub_system_no(&self) -> c_int;

  unsafe fn set_sender_id(&self, sender_id: c_int);
  unsafe fn get_sender_id(&self) -> c_int;

  unsafe fn set_packet_id(&self, packet_id: c_int);
  unsafe fn get_packet_id(&self) -> c_int;

  unsafe fn set_target_info(&self, target_info: RouteInfo);
  unsafe fn get_target_info(&self, target_info: &mut RouteInfo);

  unsafe fn set_send_info(&self, send_info: RouteInfo);
  unsafe fn get_send_info(&self, send_info: &mut RouteInfo);

  unsafe fn set_error_no(&self, error_no: c_int);
  unsafe fn get_error_no(&self) -> c_int;

  unsafe fn set_error_info(&self, error_info: *const c_char);
  unsafe fn get_error_info(&self) -> *const c_char;

  unsafe fn set_return_code(&self, return_code: c_int);
  unsafe fn get_return_code(&self) -> c_int;

  unsafe fn set_content(&self, content: *mut c_void, len: c_int);
  unsafe fn get_content(&self, len: &mut c_int) -> *const c_void;

  unsafe fn set_issue_type(&self, issue_type: c_int);
  unsafe fn get_issue_type(&self) -> c_int;

  unsafe fn set_sequence_no(&self, sequence_no: c_int);
  unsafe fn get_sequence_no(&self) -> c_int;

  unsafe fn set_key_info(&self, key_data: *mut c_void, len: c_int);
  unsafe fn get_key_info(&self, len: &mut c_int) -> *const c_void;

  unsafe fn set_app_data(&self, app_data: *const c_void, app_len: c_int);
  unsafe fn get_app_data(&self, app_len: &mut c_int) -> *const c_void;

  unsafe fn change_req_2_ans_message(&self) -> c_int;

  unsafe fn get_buff(&self, buff_len: &mut c_int) -> *mut c_void;
  unsafe fn set_buff(&self, buff: *const c_void, buff_len: c_int) -> c_int;

  unsafe fn reset(&self);

  unsafe fn set_company_id(&self, company_id: c_int);
  unsafe fn get_company_id(&self) -> c_int;

  unsafe fn set_sender_company_id(&self, sender_company_id: c_int);
  unsafe fn get_sender_company_id(&self) -> c_int;

  unsafe fn set_internal_license(&self, internal_license: c_int);
  unsafe fn get_internal_license(&self) -> c_int;

  unsafe fn set_app_reserved(&self, app_reserved: *const c_void, len: c_int);
  unsafe fn get_app_reserved(&self, len: &mut c_int) -> *const c_void;
}

#[repr(C)]
pub(crate) struct BizMessageVTable {
  pub iknown: IKnownVTable,
  pub set_function: unsafe extern "C" fn(this: *mut c_void, function_no: c_int),
  pub get_function: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_packet_type: unsafe extern "C" fn(this: *mut c_void, packet_type: c_int),
  pub get_packet_type: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_branch_no: unsafe extern "C" fn(this: *mut c_void, branch_no: c_int),
  pub get_branch_no: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_system_no: unsafe extern "C" fn(this: *mut c_void, system_no: c_int),
  pub get_system_no: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_sub_system_no: unsafe extern "C" fn(this: *mut c_void, sub_system_no: c_int),
  pub get_sub_system_no: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_sender_id: unsafe extern "C" fn(this: *mut c_void, sender_id: c_int),
  pub get_sender_id: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_packet_id: unsafe extern "C" fn(this: *mut c_void, packet_id: c_int),
  pub get_packet_id: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_target_info: unsafe extern "C" fn(this: *mut c_void, target_info: RouteInfo),
  pub get_target_info: unsafe extern "C" fn(this: *mut c_void, target_info: *mut RouteInfo),
  pub set_send_info: unsafe extern "C" fn(this: *mut c_void, send_info: RouteInfo),
  pub get_send_info: unsafe extern "C" fn(this: *mut c_void, send_info: *mut RouteInfo),
  pub set_error_no: unsafe extern "C" fn(this: *mut c_void, error_no: c_int),
  pub get_error_no: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_error_info: unsafe extern "C" fn(this: *mut c_void, error_info: *const c_char),
  pub get_error_info: unsafe extern "C" fn(this: *mut c_void) -> *const c_char,
  pub set_return_code: unsafe extern "C" fn(this: *mut c_void, return_code: c_int),
  pub get_return_code: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_content: unsafe extern "C" fn(this: *mut c_void, content: *mut c_void, len: c_int),
  pub get_content: unsafe extern "C" fn(this: *mut c_void, len: *mut c_int) -> *const c_void,
  pub set_issue_type: unsafe extern "C" fn(this: *mut c_void, issue_type: c_int),
  pub get_issue_type: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_sequence_no: unsafe extern "C" fn(this: *mut c_void, sequence_no: c_int),
  pub get_sequence_no: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_key_info: unsafe extern "C" fn(this: *mut c_void, key_data: *mut c_void, len: c_int),
  pub get_key_info: unsafe extern "C" fn(this: *mut c_void, len: *mut c_int) -> *const c_void,
  pub set_app_data: unsafe extern "C" fn(this: *mut c_void, app_data: *const c_void, app_len: c_int),
  pub get_app_data: unsafe extern "C" fn(this: *mut c_void, app_len: *mut c_int) -> *const c_void,
  pub change_req_2_ans_message: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub get_buff: unsafe extern "C" fn(this: *mut c_void, buff_len: *mut c_int) -> *mut c_void,
  pub set_buff: unsafe extern "C" fn(this: *mut c_void, buff: *const c_void, buff_len: c_int) -> c_int,
  pub reset: unsafe extern "C" fn(this: *mut c_void),
  pub set_company_id: unsafe extern "C" fn(this: *mut c_void, company_id: c_int),
  pub get_company_id: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_sender_company_id: unsafe extern "C" fn(this: *mut c_void, sender_company_id: c_int),
  pub get_sender_company_id: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_internal_license: unsafe extern "C" fn(this: *mut c_void, internal_license: c_int),
  pub get_internal_license: unsafe extern "C" fn(this: *mut c_void) -> c_int,
  pub set_app_reserved: unsafe extern "C" fn(this: *mut c_void, app_reserved: *const c_void, len: c_int),
  pub get_app_reserved: unsafe extern "C" fn(this: *mut c_void, len: *mut c_int) -> *const c_void,
}

#[repr(C)]
pub struct IBizMessageRust {
  vtable: *const BizMessageVTable,
}
impl IKnown for IBizMessageRust {
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
impl IBizMessage for IBizMessageRust {
  unsafe fn set_function(&self, function_no: c_int) {
    ((*self.vtable).set_function)(self as *const _ as *mut c_void, function_no)
  }
  unsafe fn get_function(&self) -> c_int {
    ((*self.vtable).get_function)(self as *const _ as *mut c_void)
  }
  unsafe fn set_packet_type(&self, packet_type: c_int) {
    ((*self.vtable).set_packet_type)(self as *const _ as *mut c_void, packet_type)
  }
  unsafe fn get_packet_type(&self) -> c_int {
    ((*self.vtable).get_packet_type)(self as *const _ as *mut c_void)
  }
  unsafe fn set_branch_no(&self, branch_no: c_int) {
    ((*self.vtable).set_branch_no)(self as *const _ as *mut c_void, branch_no)
  }
  unsafe fn get_branch_no(&self) -> c_int {
    ((*self.vtable).get_branch_no)(self as *const _ as *mut c_void)
  }
  unsafe fn set_system_no(&self, system_no: c_int) {
    ((*self.vtable).set_system_no)(self as *const _ as *mut c_void, system_no)
  }
  unsafe fn get_system_no(&self) -> c_int {
    ((*self.vtable).get_system_no)(self as *const _ as *mut c_void)
  }
  unsafe fn set_sub_system_no(&self, sub_system_no: c_int) {
    ((*self.vtable).set_sub_system_no)(self as *const _ as *mut c_void, sub_system_no)
  }
  unsafe fn get_sub_system_no(&self) -> c_int {
    ((*self.vtable).get_sub_system_no)(self as *const _ as *mut c_void)
  }
  unsafe fn set_sender_id(&self, sender_id: c_int) {
    ((*self.vtable).set_sender_id)(self as *const _ as *mut c_void, sender_id)
  }
  unsafe fn get_sender_id(&self) -> c_int {
    ((*self.vtable).get_sender_id)(self as *const _ as *mut c_void)
  }
  unsafe fn set_packet_id(&self, packet_id: c_int) {
    ((*self.vtable).set_packet_id)(self as *const _ as *mut c_void, packet_id)
  }
  unsafe fn get_packet_id(&self) -> c_int {
    ((*self.vtable).get_packet_id)(self as *const _ as *mut c_void)
  }
  unsafe fn set_target_info(&self, target_info: RouteInfo) {
    ((*self.vtable).set_target_info)(self as *const _ as *mut c_void, target_info)
  }
  unsafe fn get_target_info(&self, target_info: &mut RouteInfo) {
    ((*self.vtable).get_target_info)(self as *const _ as *mut c_void, target_info)
  }
  unsafe fn set_send_info(&self, send_info: RouteInfo) {
    ((*self.vtable).set_send_info)(self as *const _ as *mut c_void, send_info)
  }
  unsafe fn get_send_info(&self, send_info: &mut RouteInfo) {
    ((*self.vtable).get_send_info)(self as *const _ as *mut c_void, send_info)
  }
  unsafe fn set_error_no(&self, error_no: c_int) {
    ((*self.vtable).set_error_no)(self as *const _ as *mut c_void, error_no)
  }
  unsafe fn get_error_no(&self) -> c_int {
    ((*self.vtable).get_error_no)(self as *const _ as *mut c_void)
  }
  unsafe fn set_error_info(&self, error_info: *const c_char) {
    ((*self.vtable).set_error_info)(self as *const _ as *mut c_void, error_info)
  }
  unsafe fn get_error_info(&self) -> *const c_char {
    ((*self.vtable).get_error_info)(self as *const _ as *mut c_void)
  }
  unsafe fn set_return_code(&self, return_code: c_int) {
    ((*self.vtable).set_return_code)(self as *const _ as *mut c_void, return_code)
  }
  unsafe fn get_return_code(&self) -> c_int {
    ((*self.vtable).get_return_code)(self as *const _ as *mut c_void)
  }
  unsafe fn set_content(&self, content: *mut c_void, len: c_int) {
    ((*self.vtable).set_content)(self as *const _ as *mut c_void, content, len)
  }
  unsafe fn get_content(&self, len: &mut c_int) -> *const c_void {
    ((*self.vtable).get_content)(self as *const _ as *mut c_void, len)
  }
  unsafe fn set_issue_type(&self, issue_type: c_int) {
    ((*self.vtable).set_issue_type)(self as *const _ as *mut c_void, issue_type)
  }
  unsafe fn get_issue_type(&self) -> c_int {
    ((*self.vtable).get_issue_type)(self as *const _ as *mut c_void)
  }
  unsafe fn set_sequence_no(&self, sequence_no: c_int) {
    ((*self.vtable).set_sequence_no)(self as *const _ as *mut c_void, sequence_no)
  }
  unsafe fn get_sequence_no(&self) -> c_int {
    ((*self.vtable).get_sequence_no)(self as *const _ as *mut c_void)
  }
  unsafe fn set_key_info(&self, key_data: *mut c_void, len: c_int) {
    ((*self.vtable).set_key_info)(self as *const _ as *mut c_void, key_data, len)
  }
  unsafe fn get_key_info(&self, len: &mut c_int) -> *const c_void {
    ((*self.vtable).get_key_info)(self as *const _ as *mut c_void, len)
  }
  unsafe fn set_app_data(&self, app_data: *const c_void, app_len: c_int) {
    ((*self.vtable).set_app_data)(self as *const _ as *mut c_void, app_data, app_len)
  }
  unsafe fn get_app_data(&self, app_len: &mut c_int) -> *const c_void {
    ((*self.vtable).get_app_data)(self as *const _ as *mut c_void, app_len)
  }
  unsafe fn change_req_2_ans_message(&self) -> c_int {
    ((*self.vtable).change_req_2_ans_message)(self as *const _ as *mut c_void)
  }
  unsafe fn get_buff(&self, buff_len: &mut c_int) -> *mut c_void {
    ((*self.vtable).get_buff)(self as *const _ as *mut c_void, buff_len)
  }
  unsafe fn set_buff(&self, buff: *const c_void, buff_len: c_int) -> c_int {
    ((*self.vtable).set_buff)(self as *const _ as *mut c_void, buff, buff_len)
  }
  unsafe fn reset(&self) {
    ((*self.vtable).reset)(self as *const _ as *mut c_void)
  }
  unsafe fn set_company_id(&self, company_id: c_int) {
    ((*self.vtable).set_company_id)(self as *const _ as *mut c_void, company_id)
  }
  unsafe fn get_company_id(&self) -> c_int {
    ((*self.vtable).get_company_id)(self as *const _ as *mut c_void)
  }
  unsafe fn set_sender_company_id(&self, sender_company_id: c_int) {
    ((*self.vtable).set_sender_company_id)(self as *const _ as *mut c_void, sender_company_id)
  }
  unsafe fn get_sender_company_id(&self) -> c_int {
    ((*self.vtable).get_sender_company_id)(self as *const _ as *mut c_void)
  }
  unsafe fn set_internal_license(&self, internal_license: c_int) {
    ((*self.vtable).set_internal_license)(self as *const _ as *mut c_void, internal_license)
  }
  unsafe fn get_internal_license(&self) -> c_int {
    ((*self.vtable).get_internal_license)(self as *const _ as *mut c_void)
  }
  unsafe fn set_app_reserved(&self, app_reserved: *const c_void, len: c_int) {
    ((*self.vtable).set_app_reserved)(self as *const _ as *mut c_void, app_reserved, len)
  }
  unsafe fn get_app_reserved(&self, len: &mut c_int) -> *const c_void {
    ((*self.vtable).get_app_reserved)(self as *const _ as *mut c_void, len)
  }
}

#[napi]
pub struct BizMessage {
  ptr: *const IBizMessageRust,
}

impl BizMessage {
  pub fn new() -> Result<Self> {
    let lib = crate::get_library()?;

    unsafe {
      let ptr = (lib.new_biz_message)();
      if ptr.is_null() {
        return Err(napi::Error::from_reason("Failed to create biz message instance"));
      }

      Ok(BizMessage { ptr })
    }
  }
  pub fn new_form_ptr(ptr: *const IBizMessageRust) -> Self {
    let message = BizMessage { ptr };
    unsafe {
      (*ptr).add_ref();
    }
    return message;
  }

  fn check_ptr(&self) -> Result<&IBizMessageRust> {
    unsafe {
      if self.ptr.is_null() {
        return Err(napi::Error::from_reason("Biz message pointer is null"));
      }
      Ok(&*self.ptr)
    }
  }

  fn to_c_string(s: String, field: &str) -> Result<CString> {
    CString::new(s).map_err(|e| napi::Error::from_reason(format!("Invalid {}: {}", field, e)))
  }
  // get ptr
  pub fn get_ptr(&self) -> *const IBizMessageRust {
    self.ptr
  }
}

#[napi(object)]
pub struct JSRouteInfo {
  pub ospf_name: String,
  pub nbr_name: String,
  pub svr_name: String,
  pub plugin_id: String,
  pub connect_id: i32,
  pub member_no: i32,
}

fn copy_to_c_array(src: &str, dst: &mut [c_char]) {
  let bytes = src.as_bytes();
  for (i, &byte) in bytes.iter().enumerate() {
    if i < dst.len() - 1 {
      dst[i] = byte as c_char;
    }
  }
  if bytes.len() < dst.len() {
    dst[bytes.len()] = 0;
  } else {
    dst[dst.len() - 1] = 0;
  }
}
fn c_array_to_string(array: &[c_char]) -> Result<String> {
  unsafe {
    let cstr = CStr::from_ptr(array.as_ptr());

    match cstr.to_str() {
      Ok(s) => Ok(s.to_string()),
      Err(e) => Err(Error::new(Status::InvalidArg, format!("Invalid UTF-8 sequence: {}", e))),
    }
  }
}

#[napi]
impl BizMessage {
  /// 设置功能号
  #[napi]
  pub fn set_function(&self, function_no: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_function(function_no);
    }
    Ok(())
  }
  /// 获取功能号
  #[napi]
  pub fn get_function(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_function() };
    Ok(result)
  }
  /// 设置包类型
  #[napi]
  pub fn set_packet_type(&self, packet_type: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_packet_type(packet_type);
    }
    Ok(())
  }
  /// 获取包类型
  #[napi]
  pub fn get_packet_type(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_packet_type() };
    Ok(result)
  }
  /// 设置营业部号
  #[napi]
  pub fn set_branch_no(&self, branch_no: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_branch_no(branch_no);
    }
    Ok(())
  }
  /// 获取营业部号
  #[napi]
  pub fn get_branch_no(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_branch_no() };
    Ok(result)
  }
  /// 设置系统号
  #[napi]
  pub fn set_system_no(&self, system_no: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_system_no(system_no);
    }
    Ok(())
  }
  /// 获取系统号
  #[napi]
  pub fn get_system_no(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_system_no() };
    Ok(result)
  }
  /// 设置子系统号
  #[napi]
  pub fn set_sub_system_no(&self, sub_system_no: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_sub_system_no(sub_system_no);
    }
    Ok(())
  }
  /// 获取子系统号
  #[napi]
  pub fn get_sub_system_no(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_sub_system_no() };
    Ok(result)
  }
  /// 设置发送者编号
  #[napi]
  pub fn set_sender_id(&self, sender_id: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_sender_id(sender_id);
    }
    Ok(())
  }
  /// 获取发送者编号
  #[napi]
  pub fn get_sender_id(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_sender_id() };
    Ok(result)
  }
  /// 设置包序号
  #[napi]
  pub fn set_packet_id(&self, packet_id: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_packet_id(packet_id);
    }
    Ok(())
  }
  /// 获取包序号
  #[napi]
  pub fn get_packet_id(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_packet_id() };
    Ok(result)
  }
  /// 设置目的地路由
  #[napi]
  pub fn set_target_info(&self, js_target_info: JSRouteInfo) -> Result<()> {
    let mut target_info = RouteInfo::default();
    copy_to_c_array(&js_target_info.ospf_name, &mut target_info.ospf_name);
    copy_to_c_array(&js_target_info.nbr_name, &mut target_info.nbr_name);
    copy_to_c_array(&js_target_info.svr_name, &mut target_info.svr_name);
    copy_to_c_array(&js_target_info.plugin_id, &mut target_info.plugin_id);

    target_info.connect_id = js_target_info.connect_id;
    target_info.member_no = js_target_info.member_no;

    unsafe {
      self.check_ptr()?.set_target_info(target_info);
    }
    Ok(())
  }
  /// 获取目的地路由
  #[napi]
  pub fn get_target_info(&self) -> Result<JSRouteInfo> {
    let mut target_info = RouteInfo::default();

    unsafe {
      self.check_ptr()?.get_target_info(&mut target_info);
      let ospf_name = c_array_to_string(&target_info.ospf_name)?;
      let nbr_name = c_array_to_string(&target_info.nbr_name)?;
      let svr_name = c_array_to_string(&target_info.svr_name)?;
      let plugin_id = c_array_to_string(&target_info.plugin_id)?;

      Ok(JSRouteInfo {
        ospf_name,
        nbr_name,
        svr_name,
        plugin_id,
        connect_id: target_info.connect_id,
        member_no: target_info.member_no,
      })
    }
  }
  /// 设置发送者路由
  #[napi]
  pub fn set_send_info(&self, js_send_info: JSRouteInfo) -> Result<()> {
    let mut send_info = RouteInfo::default();
    copy_to_c_array(&js_send_info.ospf_name, &mut send_info.ospf_name);
    copy_to_c_array(&js_send_info.nbr_name, &mut send_info.nbr_name);
    copy_to_c_array(&js_send_info.svr_name, &mut send_info.svr_name);
    copy_to_c_array(&js_send_info.plugin_id, &mut send_info.plugin_id);
    send_info.connect_id = js_send_info.connect_id;
    send_info.member_no = js_send_info.member_no;
    unsafe {
      self.check_ptr()?.set_send_info(send_info);
    }
    Ok(())
  }
  /// 获取发送者路由
  #[napi]
  pub fn get_send_info(&self) -> Result<JSRouteInfo> {
    let mut send_info = RouteInfo::default();
    unsafe {
      self.check_ptr()?.get_send_info(&mut send_info);
    }
    let js_send_info = JSRouteInfo {
      ospf_name: c_array_to_string(&send_info.ospf_name)?,
      nbr_name: c_array_to_string(&send_info.nbr_name)?,
      svr_name: c_array_to_string(&send_info.svr_name)?,
      plugin_id: c_array_to_string(&send_info.plugin_id)?,
      connect_id: send_info.connect_id,
      member_no: send_info.member_no,
    };
    Ok(js_send_info)
  }
  /// 设置错误号
  #[napi]
  pub fn set_error_no(&self, error_no: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_error_no(error_no);
    }
    Ok(())
  }
  /// 获取错误号
  #[napi]
  pub fn get_error_no(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_error_no() };
    Ok(result)
  }
  /// 设置错误信息
  #[napi]
  pub fn set_error_info(&self, error_info: String) -> Result<()> {
    let c_error_info = Self::to_c_string(error_info, "error_info")?;
    unsafe {
      self.check_ptr()?.set_error_info(c_error_info.as_ptr());
    }
    Ok(())
  }
  /// 获取错误信息
  #[napi]
  pub fn get_error_info(&self) -> Result<String> {
    let c_error_info = unsafe { self.check_ptr()?.get_error_info() };
    let bytes = unsafe {
      if c_error_info.is_null() {
        &[]
      } else {
        CStr::from_ptr(c_error_info).to_bytes()
      }
    };
    let utf8 = encoding::all::GBK.decode(bytes, encoding::DecoderTrap::Strict).unwrap();
    Ok(utf8)
  }
  /// 设置返回码
  #[napi]
  pub fn set_return_code(&self, return_code: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_return_code(return_code);
    }
    Ok(())
  }
  /// 获取返回码
  #[napi]
  pub fn get_return_code(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_return_code() };
    Ok(result)
  }
  /// 设置业务内容
  #[napi]
  pub fn set_content(&self, content: Buffer) -> Result<()> {
    let content = content.as_ref();
    unsafe {
      self
        .check_ptr()?
        .set_content(content.as_ptr() as *mut c_void, content.len() as c_int);
    }
    Ok(())
  }
  /// 获取业务内容
  #[napi]
  pub fn get_content(&self) -> Result<Buffer> {
    let mut len = 0;
    let content = unsafe {
      let content = self.check_ptr()?.get_content(&mut len);
      Buffer::from(std::slice::from_raw_parts(content as *const u8, len as usize))
    };
    Ok(content)
  }
  /// 以下接口用于消息中心1.0的订阅
  /// 设置订阅类型
  #[napi]
  pub fn set_issue_type(&self, issue_type: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_issue_type(issue_type);
    }
    Ok(())
  }
  /// 获取订阅类型
  #[napi]
  pub fn get_issue_type(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_issue_type() };
    Ok(result)
  }
  /// 设置序号
  #[napi]
  pub fn set_sequence_no(&self, sequence_no: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_sequence_no(sequence_no);
    }
    Ok(())
  }
  /// 获取序号
  #[napi]
  pub fn get_sequence_no(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_sequence_no() };
    Ok(result)
  }
  /// 设置关键字段信息
  #[napi]
  pub fn set_key_info(&self, key_data: Buffer) -> Result<()> {
    let key_data = key_data.as_ref();
    unsafe {
      self
        .check_ptr()?
        .set_key_info(key_data.as_ptr() as *mut c_void, key_data.len() as c_int);
    }
    Ok(())
  }
  /// 获取关键字段信息
  #[napi]
  pub fn get_key_info(&self) -> Result<Buffer> {
    let mut len = 0;
    let key_info = unsafe {
      let key_info = self.check_ptr()?.get_key_info(&mut len);
      Buffer::from(std::slice::from_raw_parts(key_info as *const u8, len as usize))
    };
    Ok(key_info)
  }
  /// 设置附加数据，订阅推送时原样返回
  #[napi]
  pub fn set_app_data(&self, app_data: Buffer) -> Result<()> {
    let app_data = app_data.as_ref();
    unsafe {
      self
        .check_ptr()?
        .set_app_data(app_data.as_ptr() as *const c_void, app_data.len() as c_int);
    }
    Ok(())
  }
  /// 获取附加数据，订阅推送时原样返回
  #[napi]
  pub fn get_app_data(&self) -> Result<Buffer> {
    let mut len = 0;
    let app_data = unsafe {
      let app_data = self.check_ptr()?.get_app_data(&mut len);
      Buffer::from(std::slice::from_raw_parts(app_data as *const u8, len as usize))
    };
    Ok(app_data)
  }
  /// 请求转应答
  #[napi]
  pub fn change_req_2_ans_message(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.change_req_2_ans_message() };
    Ok(result)
  }
  /// 获取二进制
  #[napi]
  pub fn get_buff(&self) -> Result<Buffer> {
    let mut len = 0;
    let buff = unsafe {
      let buff = self.check_ptr()?.get_buff(&mut len);
      Buffer::from(std::slice::from_raw_parts(buff as *const u8, len as usize))
    };
    Ok(buff)
  }
  /// 设置二进制
  #[napi]
  pub fn set_buff(&self, buff: Buffer) -> Result<()> {
    let buff = buff.as_ref();
    unsafe {
      self.check_ptr()?.set_buff(buff.as_ptr() as *const c_void, buff.len() as c_int);
    }
    Ok(())
  }
  /// 清除消息内的字段，可以下次复用。
  #[napi]
  pub fn reset(&self) -> Result<()> {
    unsafe {
      self.check_ptr()?.reset();
    }
    Ok(())
  }
  /// 设置公司编号
  #[napi]
  pub fn set_company_id(&self, company_id: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_company_id(company_id);
    }
    Ok(())
  }
  /// 获取公司编号
  #[napi]
  pub fn get_company_id(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_company_id() };
    Ok(result)
  }
  /// 设置发送者公司编号
  #[napi]
  pub fn set_sender_company_id(&self, sender_company_id: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_sender_company_id(sender_company_id);
    }
    Ok(())
  }
  /// 获取发送者公司编号
  #[napi]
  pub fn get_sender_company_id(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_sender_company_id() };
    Ok(result)
  }
  #[napi]
  pub fn set_internal_license(&self, internal_license: i32) -> Result<()> {
    unsafe {
      self.check_ptr()?.set_internal_license(internal_license);
    }
    Ok(())
  }
  #[napi]
  pub fn get_internal_license(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_internal_license() };
    Ok(result)
  }
  #[napi]
  pub fn set_app_reserved(&self, app_reserved: Buffer) -> Result<()> {
    let app_reserved = app_reserved.as_ref();
    unsafe {
      self
        .check_ptr()?
        .set_app_reserved(app_reserved.as_ptr() as *const c_void, app_reserved.len() as c_int);
    }
    Ok(())
  }
  #[napi]
  pub fn get_app_reserved(&self) -> Result<Buffer> {
    let mut len = 0;
    let app_reserved = unsafe {
      let app_reserved = self.check_ptr()?.get_app_reserved(&mut len);
      Buffer::from(std::slice::from_raw_parts(app_reserved as *const u8, len as usize))
    };
    Ok(app_reserved)
  }
}

impl Drop for BizMessage {
  fn drop(&mut self) {
    unsafe {
      if !self.ptr.is_null() {
        let ptr = &*self.ptr;
        ptr.release();
        self.ptr = std::ptr::null_mut();
      }
    }
  }
}

unsafe impl Send for BizMessage {}
unsafe impl Sync for BizMessage {}
