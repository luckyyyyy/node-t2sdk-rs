use dashmap::DashMap;
use encoding::Encoding;
use napi::Result;
use std::any::Any;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::OnceLock;

use crate::config::CConfigInterfaceRust;
use crate::iknown::{IKnown, IKnownVTable};

use crate::biz_message::{IBizMessage, IBizMessageRust, RouteInfo};
use crate::callback::{create_callback, CCallbackRust};
use crate::f2_packer::IF2Packer;
use crate::f2_unpacker::IF2UnPackerRust;

pub enum RecvBizResult {
  UnPacker(IF2UnPackerRust), // 成功情况下的解包器
  ErrorString(String),       // 错误信息字符串
  None,                      // 对应解包失败的情况
}

#[repr(u32)]
pub enum ConnectionStatus {
  Disconnected = 0x0000,   // 未连接
  Connecting = 0x0001,     // socket正在连接
  Connected = 0x0002,      // socket已连接
  SafeConnecting = 0x0004, // 正在建立安全连接
  SafeConnected = 0x0008,  // 已建立安全连接
  Registering = 0x0010,    // 正注册
  Registered = 0x0020,     // 已注册
  Rejected = 0x0040,       // 被拒绝,将被关闭
}

#[repr(C)]
pub struct ReqData {
  sequence_no: i32,
  issue_type: i32,
  key_info: *mut std::ffi::c_void,
  key_info_len: i32,
  file_head: *mut std::ffi::c_void,
  file_head_len: i32,
  packet_type: i32,
  route_info: RouteInfo,
  sub_system_no: i32,
  company_id: i32,
}

#[repr(u32)]
pub enum RecvFlags {
  JustRemoveHandle = 0x0001,
}

pub(crate) trait CConnectionInterface: IKnown {
  // 初始化连接对象
  unsafe fn create(&mut self, callback: Option<*mut c_void>) -> i32;
  // 开始连接/注册
  unsafe fn connect(&mut self, timeout: u32) -> i32;
  // 断开连接
  unsafe fn close(&mut self) -> i32;
  unsafe fn reserved1(&self) -> i32;
  unsafe fn reserved2(&self) -> i32;
  unsafe fn reserved3(&self) -> i32;
  unsafe fn reserved4(&self) -> i32;
  unsafe fn reserved5(&self) -> i32;
  unsafe fn reserved6(&self) -> i32;
  // 获取服务器地址和端口
  unsafe fn get_server_address(&self) -> (&str, Option<i32>);
  // 获取连接状态
  unsafe fn get_status(&self) -> i32;
  // 获取服务器负载
  unsafe fn get_server_load(&self) -> i32;
  // 获取错误信息
  unsafe fn get_error_msg(&self, error_code: i32) -> String;
  // 获取连接错误号
  unsafe fn get_connect_error(&self) -> i32;
  // 发送业务数据
  unsafe fn send_biz(&mut self, fun_id: i32, packer: &dyn IF2Packer, asy: i32, system_no: i32, compress_id: i32) -> i32;
  // 接收业务数据
  unsafe fn recv_biz(&mut self, handle: i32, unpacker_or_str: &mut Option<Box<dyn Any>>, timeout: u32, flag: u32) -> i32;
  unsafe fn send_biz_ex(
    &self,
    fun_id: i32,
    packer: &mut dyn IF2Packer,
    svr_name: &str,
    asy: i32,
    system_no: i32,
    compress_id: i32,
    branch_no: i32,
    request: Option<&ReqData>,
  ) -> i32;
  unsafe fn send_biz_ex2(
    &self,
    fun_id: i32,
    packer: &mut dyn IF2Packer,
    svr_name: &str,
    asy: i32,
    system_no: i32,
    compress_id: i32,
    branch_no: i32,
    request: Option<&ReqData>,
    timeout: u32,
    flag: u32,
  ) -> i32;
  unsafe fn create_ex(&self, callback: Option<*mut c_void>) -> i32;
  unsafe fn get_real_address(&self) -> &str;
  unsafe fn reserved8(&self) -> i32;
  unsafe fn reserved9(&self) -> i32;
  unsafe fn get_self_address(&self) -> &str;
  unsafe fn get_self_mac(&self) -> &str;

  // virtual CSubscribeInterface* FUNCTION_CALL_MODE NewSubscriber(CSubCallbackInterface *lpCallback,char* SubScribeName,int iTimeOut,
  //   int iInitRecvQLen=INIT_RECVQ_LEN,int iStepRecvQLen=STEP_RECVQ_LEN) = 0;
  unsafe fn new_subscriber(
    &self,
    callback: *mut c_void,
    subscribe_name: &str,
    timeout: i32,
    init_recv_q_len: i32,
    step_recv_q_len: i32,
  ) -> i32;
  // virtual CPublishInterface* FUNCTION_CALL_MODE NewPublisher(char* PublishName,int msgCount,int iTimeOut,bool bResetNo = false) = 0;
  unsafe fn new_publisher(&self, publish_name: &str, msg_count: i32, timeout: i32, reset_no: bool) -> i32;
  // virtual IF2UnPacker* FUNCTION_CALL_MODE GetTopic(bool byForce,int iTimeOut) = 0;
  unsafe fn get_topic(&self, by_force: bool, timeout: i32) -> i32;
  // virtual const char* FUNCTION_CALL_MODE GetMCLastError() = 0;
  unsafe fn get_mc_last_error(&self) -> &str;
  // virtual int FUNCTION_CALL_MODE Create2BizMsg(CCallbackRust *lpCallback) = 0;
  unsafe fn create_2_biz_msg(&self, callback: *mut CCallbackRust) -> i32;
  // virtual int FUNCTION_CALL_MODE SendBizMsg(IBizMessage* lpMsg,int nAsy = 0) = 0;
  unsafe fn send_biz_msg(&self, msg: *const IBizMessageRust, asy: i32) -> i32;
  // virtual int FUNCTION_CALL_MODE RecvBizMsg(int hSend, IBizMessage** lpMsg, unsigned uiTimeout = 1000, unsigned uiFlag = 0) = 0;
  unsafe fn recv_biz_msg(&self, send: i32, msg: &mut Option<Box<dyn IBizMessage>>, timeout: u32, flag: u32) -> i32;
  // virtual CFileUpdateInterface* FUNCTION_CALL_MODE NewFileUpdate(const char* szTopicName,CFileUpdateCallbackInterface* lpCallBack ,const char* szScanDir,const char* szUpdateDir,unsigned int uiTimeOut = 5000, const char * szDirFilter = NULL) = 0;
  unsafe fn new_file_update(
    &self,
    topic_name: &str,
    callback: *mut c_void,
    scan_dir: &str,
    update_dir: &str,
    timeout: u32,
    dir_filter: Option<&str>,
  ) -> i32;
  // virtual const char* FUNCTION_CALL_MODE GetFileUpdateLastError() = 0;
  unsafe fn get_file_update_last_error(&self) -> &str;
  // virtual const char * FUNCTION_CALL_MODE GetLastAnsError(bool bAsyError = 0) = 0;
  unsafe fn get_last_ans_error(&self, asy_error: bool) -> &str;
  // virtual CSubscribeInterface* FUNCTION_CALL_MODE NewSubscriberEx(CSubCallbackInterface *lpCallback,char* SubScribeName,SUB_ROUTER_INFO &subRoterInfo,int iTimeOut,
  //   int iInitRecvQLen=INIT_RECVQ_LEN,int iStepRecvQLen=STEP_RECVQ_LEN) = 0;
  unsafe fn new_subscriber_ex(
    &self,
    callback: *mut c_void,
    subscribe_name: &str,
    sub_router_info: &std::ffi::c_void,
    timeout: i32,
    init_recv_q_len: i32,
    step_recv_q_len: i32,
  ) -> i32;

  // virtual CSubscribeInterface* FUNCTION_CALL_MODE NewClusterSubscriber(CSubCallbackInterface *lpCallback,char* SubScribeName,SUB_ROUTER_INFO &subRoterInfo,int iTimeOut,
  //   int iInitRecvQLen=INIT_RECVQ_LEN,int iStepRecvQLen=STEP_RECVQ_LEN) = 0;
  unsafe fn new_cluster_subscriber(
    &self,
    callback: *mut c_void,
    subscribe_name: &str,
    sub_router_info: &std::ffi::c_void,
    timeout: i32,
    init_recv_q_len: i32,
    step_recv_q_len: i32,
  ) -> i32;
  // virtual const char * FUNCTION_CALL_MODE GetServerNodeName() = 0;
  unsafe fn get_server_node_name(&self) -> &str;
  // virtual void FUNCTION_CALL_MODE RecycleDataBuf(void *lpData) = 0;
  unsafe fn recycle_data_buf(&self, data: &std::ffi::c_void);
  // virtual void FUNCTION_CALL_MODE SetUniquePrefix(const char* lpPrefix) = 0;
  unsafe fn set_unique_prefix(&self, prefix: &str);
  // virtual int FUNCTION_CALL_MODE ActiveClose(bool bSafe = false, unsigned int uiTimeout = 5000) = 0;
  unsafe fn active_close(&self, safe: bool, timeout: u32) -> i32;
  // virtual int FUNCTION_CALL_MODE SetServers(const char * szServers) = 0;
  unsafe fn set_servers(&self, servers: &str) -> i32;
}

#[repr(C)]
pub(crate) struct VTable {
  pub iknown: IKnownVTable,
  create: unsafe fn(this: *mut c_void, callback: *const c_void) -> c_int,
  connect: unsafe fn(this: *mut c_void, timeout: u32) -> c_int,
  close: unsafe fn(this: *mut c_void) -> c_int,
  reserved1: unsafe fn(this: *const c_void) -> c_int,
  reserved2: unsafe fn(this: *const c_void) -> c_int,
  reserved3: unsafe fn(this: *const c_void) -> c_int,
  reserved4: unsafe fn(this: *const c_void) -> c_int,
  reserved5: unsafe fn(this: *const c_void) -> c_int,
  reserved6: unsafe fn(this: *const c_void) -> c_int,
  get_server_address: unsafe fn(this: *const c_void, server: *mut *const c_char, port: *mut c_int) -> c_int,
  get_status: unsafe fn(this: *const c_void) -> c_int,
  get_server_load: unsafe fn(this: *const c_void) -> c_int,
  get_error_msg: unsafe fn(this: *const c_void, error_code: c_int) -> *const c_char,
  get_connect_error: unsafe fn(this: *const c_void) -> c_int,
  send_biz: unsafe fn(this: *mut c_void, fun_id: c_int, packer: *const c_void, asy: c_int, system_no: c_int, compress_id: c_int) -> c_int,
  recv_biz: unsafe fn(this: *mut c_void, handle: c_int, unpacker_or_str: *mut *mut c_void, timeout: u32, flag: u32) -> c_int,
  send_biz_ex: unsafe fn(
    this: *const c_void,
    fun_id: c_int,
    packer: *mut c_void,
    svr_name: *const c_char,
    asy: c_int,
    system_no: c_int,
    compress_id: c_int,
    branch_no: c_int,
    request: *const ReqData,
  ) -> c_int,
  send_biz_ex2: unsafe fn(
    this: *const c_void,
    fun_id: c_int,
    packer: *mut c_void,
    svr_name: *const c_char,
    asy: c_int,
    system_no: c_int,
    compress_id: c_int,
    branch_no: c_int,
    request: *const ReqData,
    timeout: u32,
    flag: u32,
  ) -> c_int,
  create_ex: unsafe fn(this: *const c_void, callback: *const c_void) -> c_int,
  get_real_address: unsafe fn(this: *const c_void) -> *const c_char,
  reserved8: unsafe fn(this: *const c_void) -> c_int,
  reserved9: unsafe fn(this: *const c_void) -> c_int,
  get_self_address: unsafe fn(this: *const c_void) -> *const c_char,
  get_self_mac: unsafe fn(this: *const c_void) -> *const c_char,
  new_subscriber: unsafe fn(
    this: *const c_void,
    callback: *const c_void,
    subscribe_name: *const c_char,
    timeout: c_int,
    init_recv_q_len: c_int,
    step_recv_q_len: c_int,
  ) -> c_int,
  new_publisher: unsafe fn(this: *const c_void, publish_name: *const c_char, msg_count: c_int, timeout: c_int, reset_no: c_int) -> c_int,
  get_topic: unsafe fn(this: *const c_void, by_force: c_int, timeout: c_int) -> c_int,
  get_mc_last_error: unsafe fn(this: *const c_void) -> *const c_char,
  create_2_biz_msg: unsafe fn(this: *const c_void, callback: *mut CCallbackRust) -> c_int,
  send_biz_msg: unsafe fn(this: *const c_void, msg: *const c_void, asy: c_int) -> c_int,
  recv_biz_msg: unsafe fn(this: *const c_void, send: c_int, msg: *mut *mut c_void, timeout: u32, flag: u32) -> c_int,
  new_file_update: unsafe fn(
    this: *const c_void,
    topic_name: *const c_char,
    callback: *const c_void,
    scan_dir: *const c_char,
    update_dir: *const c_char,
    timeout: u32,
    dir_filter: *const c_char,
  ) -> c_int,
  get_file_update_last_error: unsafe fn(this: *const c_void) -> *const c_char,
  get_last_ans_error: unsafe fn(this: *const c_void, asy_error: c_int) -> *const c_char,
  new_subscriber_ex: unsafe fn(
    this: *const c_void,
    callback: *const c_void,
    subscribe_name: *const c_char,
    sub_router_info: *const std::ffi::c_void,
    timeout: c_int,
    init_recv_q_len: c_int,
    step_recv_q_len: c_int,
  ) -> c_int,
  new_cluster_subscriber: unsafe fn(
    this: *const c_void,
    callback: *const c_void,
    subscribe_name: *const c_char,
    sub_router_info: *const c_void,
    timeout: c_int,
    init_recv_q_len: c_int,
    step_recv_q_len: c_int,
  ) -> c_int,
  get_server_node_name: unsafe fn(this: *const c_void) -> *const c_char,
  recycle_data_buf: unsafe fn(this: *const c_void, data: *const c_void),
  set_unique_prefix: unsafe fn(this: *const c_void, prefix: *const c_char),
  active_close: unsafe fn(this: *const c_void, safe: c_int, timeout: u32) -> c_int,
  set_servers: unsafe fn(this: *const c_void, servers: *const c_char) -> c_int,
}
#[repr(C)]
pub(crate) struct IConnectionRust {
  vtable: *const VTable,
}

impl IKnown for IConnectionRust {
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

impl CConnectionInterface for IConnectionRust {
  unsafe fn create(&mut self, callback: Option<*mut c_void>) -> i32 {
    ((*self.vtable).create)(
      self as *mut _ as *mut c_void,
      callback
        .as_ref()
        .map(|c| c as *const _ as *const c_void)
        .unwrap_or(std::ptr::null()),
    )
  }
  unsafe fn connect(&mut self, timeout: u32) -> i32 {
    ((*self.vtable).connect)(self as *mut _ as *mut c_void, timeout)
  }
  unsafe fn close(&mut self) -> i32 {
    ((*self.vtable).close)(self as *mut _ as *mut c_void)
  }

  unsafe fn reserved1(&self) -> i32 {
    ((*self.vtable).reserved1)(self as *const _ as *mut c_void)
  }
  unsafe fn reserved2(&self) -> i32 {
    ((*self.vtable).reserved2)(self as *const _ as *mut c_void)
  }
  unsafe fn reserved3(&self) -> i32 {
    ((*self.vtable).reserved3)(self as *const _ as *mut c_void)
  }
  unsafe fn reserved4(&self) -> i32 {
    ((*self.vtable).reserved4)(self as *const _ as *mut c_void)
  }
  unsafe fn reserved5(&self) -> i32 {
    ((*self.vtable).reserved5)(self as *const _ as *mut c_void)
  }
  unsafe fn reserved6(&self) -> i32 {
    ((*self.vtable).reserved6)(self as *const _ as *mut c_void)
  }
  unsafe fn get_server_address(&self) -> (&str, Option<i32>) {
    let mut server = std::ptr::null();
    let mut port = 0;
    ((*self.vtable).get_server_address)(self as *const _ as *mut c_void, &mut server, &mut port);
    (CStr::from_ptr(server).to_str().unwrap(), Some(port))
  }
  unsafe fn get_status(&self) -> i32 {
    ((*self.vtable).get_status)(self as *const _ as *mut c_void)
  }
  unsafe fn get_server_load(&self) -> i32 {
    ((*self.vtable).get_server_load)(self as *const _ as *mut c_void)
  }
  unsafe fn get_error_msg(&self, error_code: i32) -> String {
    let c_str = CStr::from_ptr(((*self.vtable).get_error_msg)(self as *const _ as *mut c_void, error_code));
    let bytes = c_str.to_bytes();

    match std::str::from_utf8(bytes) {
      Ok(s) => s.to_string(),
      Err(_) => encoding::all::GBK.decode(bytes, encoding::DecoderTrap::Strict).unwrap(),
    }
  }
  unsafe fn get_connect_error(&self) -> i32 {
    ((*self.vtable).get_connect_error)(self as *const _ as *mut c_void)
  }
  unsafe fn send_biz(&mut self, fun_id: i32, packer: &dyn IF2Packer, asy: i32, system_no: i32, compress_id: i32) -> i32 {
    ((*self.vtable).send_biz)(
      self as *mut _ as *mut c_void,
      fun_id,
      packer as *const _ as *const c_void,
      asy,
      system_no,
      compress_id,
    )
  }
  unsafe fn recv_biz(&mut self, handle: i32, unpacker_or_str: &mut Option<Box<dyn Any>>, timeout: u32, flag: u32) -> i32 {
    ((*self.vtable).recv_biz)(
      self as *mut _ as *mut c_void,
      handle,
      unpacker_or_str as *mut _ as *mut *mut c_void,
      timeout,
      flag,
    )
  }
  unsafe fn send_biz_ex(
    &self,
    fun_id: i32,
    packer: &mut dyn IF2Packer,
    svr_name: &str,
    asy: i32,
    system_no: i32,
    compress_id: i32,
    branch_no: i32,
    request: Option<&ReqData>,
  ) -> i32 {
    ((*self.vtable).send_biz_ex)(
      self as *const _ as *mut c_void,
      fun_id,
      packer as *mut _ as *mut c_void,
      CString::new(svr_name).unwrap().as_ptr(),
      asy,
      system_no,
      compress_id,
      branch_no,
      request.map(|r| r as *const _).unwrap_or(std::ptr::null()),
    )
  }
  unsafe fn send_biz_ex2(
    &self,
    fun_id: i32,
    packer: &mut dyn IF2Packer,
    svr_name: &str,
    asy: i32,
    system_no: i32,
    compress_id: i32,
    branch_no: i32,
    request: Option<&ReqData>,
    timeout: u32,
    flag: u32,
  ) -> i32 {
    ((*self.vtable).send_biz_ex2)(
      self as *const _ as *mut c_void,
      fun_id,
      packer as *mut _ as *mut c_void,
      CString::new(svr_name).unwrap().as_ptr(),
      asy,
      system_no,
      compress_id,
      branch_no,
      request.map(|r| r as *const _).unwrap_or(std::ptr::null()),
      timeout,
      flag,
    )
  }
  unsafe fn create_ex(&self, callback: Option<*mut c_void>) -> i32 {
    ((*self.vtable).create_ex)(
      self as *const _ as *mut c_void,
      callback
        .as_ref()
        .map(|c| c as *const _ as *const c_void)
        .unwrap_or(std::ptr::null()),
    )
  }
  unsafe fn get_real_address(&self) -> &str {
    CStr::from_ptr(((*self.vtable).get_real_address)(self as *const _ as *mut c_void))
      .to_str()
      .unwrap()
  }
  unsafe fn reserved8(&self) -> i32 {
    ((*self.vtable).reserved8)(self as *const _ as *mut c_void)
  }
  unsafe fn reserved9(&self) -> i32 {
    ((*self.vtable).reserved9)(self as *const _ as *mut c_void)
  }
  unsafe fn get_self_address(&self) -> &str {
    CStr::from_ptr(((*self.vtable).get_self_address)(self as *const _ as *mut c_void))
      .to_str()
      .unwrap()
  }
  unsafe fn get_self_mac(&self) -> &str {
    CStr::from_ptr(((*self.vtable).get_self_mac)(self as *const _ as *mut c_void))
      .to_str()
      .unwrap()
  }

  unsafe fn new_subscriber(
    &self,
    callback: *mut c_void,
    subscribe_name: &str,
    timeout: i32,
    init_recv_q_len: i32,
    step_recv_q_len: i32,
  ) -> i32 {
    ((*self.vtable).new_subscriber)(
      self as *const _ as *mut c_void,
      &callback as *const _ as *const c_void,
      CString::new(subscribe_name).unwrap().as_ptr(),
      timeout,
      init_recv_q_len,
      step_recv_q_len,
    )
  }

  unsafe fn new_publisher(&self, publish_name: &str, msg_count: i32, timeout: i32, reset_no: bool) -> i32 {
    ((*self.vtable).new_publisher)(
      self as *const _ as *mut c_void,
      CString::new(publish_name).unwrap().as_ptr(),
      msg_count,
      timeout,
      reset_no as i32,
    )
  }
  unsafe fn get_topic(&self, by_force: bool, timeout: i32) -> i32 {
    ((*self.vtable).get_topic)(self as *const _ as *mut c_void, by_force as i32, timeout)
  }
  unsafe fn get_mc_last_error(&self) -> &str {
    CStr::from_ptr(((*self.vtable).get_mc_last_error)(self as *const _ as *mut c_void))
      .to_str()
      .unwrap()
  }
  unsafe fn create_2_biz_msg(&self, callback: *mut CCallbackRust) -> i32 {
    ((*self.vtable).create_2_biz_msg)(self as *const _ as *mut c_void, callback)
  }

  unsafe fn send_biz_msg(&self, msg: *const IBizMessageRust, asy: i32) -> i32 {
    ((*self.vtable).send_biz_msg)(self as *const _ as *mut c_void, msg as *const c_void, asy)
  }
  unsafe fn recv_biz_msg(&self, send: i32, msg: &mut Option<Box<dyn IBizMessage>>, timeout: u32, flag: u32) -> i32 {
    ((*self.vtable).recv_biz_msg)(
      self as *const _ as *mut c_void,
      send,
      msg as *mut _ as *mut *mut c_void,
      timeout,
      flag,
    )
  }
  unsafe fn new_file_update(
    &self,
    topic_name: &str,
    callback: *mut c_void,
    scan_dir: &str,
    update_dir: &str,
    timeout: u32,
    dir_filter: Option<&str>,
  ) -> i32 {
    ((*self.vtable).new_file_update)(
      self as *const _ as *mut c_void,
      CString::new(topic_name).unwrap().as_ptr(),
      &callback as *const _ as *const c_void,
      CString::new(scan_dir).unwrap().as_ptr(),
      CString::new(update_dir).unwrap().as_ptr(),
      timeout,
      dir_filter.map(|d| CString::new(d).unwrap().as_ptr()).unwrap_or(std::ptr::null()),
    )
  }
  unsafe fn get_file_update_last_error(&self) -> &str {
    CStr::from_ptr(((*self.vtable).get_file_update_last_error)(self as *const _ as *mut c_void))
      .to_str()
      .unwrap()
  }
  unsafe fn get_last_ans_error(&self, asy_error: bool) -> &str {
    CStr::from_ptr(((*self.vtable).get_last_ans_error)(
      self as *const _ as *mut c_void,
      asy_error as i32,
    ))
    .to_str()
    .unwrap()
  }
  unsafe fn new_subscriber_ex(
    &self,
    callback: *mut c_void,
    subscribe_name: &str,
    sub_router_info: &std::ffi::c_void,
    timeout: i32,
    init_recv_q_len: i32,
    step_recv_q_len: i32,
  ) -> i32 {
    ((*self.vtable).new_subscriber_ex)(
      self as *const _ as *mut c_void,
      &callback as *const _ as *const c_void,
      CString::new(subscribe_name).unwrap().as_ptr(),
      sub_router_info,
      timeout,
      init_recv_q_len,
      step_recv_q_len,
    )
  }
  unsafe fn new_cluster_subscriber(
    &self,
    callback: *mut c_void,
    subscribe_name: &str,
    sub_router_info: &std::ffi::c_void,
    timeout: i32,
    init_recv_q_len: i32,
    step_recv_q_len: i32,
  ) -> i32 {
    ((*self.vtable).new_cluster_subscriber)(
      self as *const _ as *mut c_void,
      &callback as *const _ as *const c_void,
      CString::new(subscribe_name).unwrap().as_ptr(),
      sub_router_info,
      timeout,
      init_recv_q_len,
      step_recv_q_len,
    )
  }

  unsafe fn get_server_node_name(&self) -> &str {
    CStr::from_ptr(((*self.vtable).get_server_node_name)(self as *const _ as *mut c_void))
      .to_str()
      .unwrap()
  }
  unsafe fn recycle_data_buf(&self, data: &std::ffi::c_void) {
    ((*self.vtable).recycle_data_buf)(self as *const _ as *mut c_void, data)
  }
  unsafe fn set_unique_prefix(&self, prefix: &str) {
    ((*self.vtable).set_unique_prefix)(self as *const _ as *mut c_void, CString::new(prefix).unwrap().as_ptr())
  }
  unsafe fn active_close(&self, safe: bool, timeout: u32) -> i32 {
    ((*self.vtable).active_close)(self as *const _ as *mut c_void, safe as i32, timeout)
  }
  unsafe fn set_servers(&self, servers: &str) -> i32 {
    ((*self.vtable).set_servers)(self as *const _ as *mut c_void, CString::new(servers).unwrap().as_ptr())
  }
}

type MessageCallback = Box<dyn FnOnce(*mut IBizMessageRust) + Send>;

pub struct CallbackRegistry {
  callbacks: DashMap<(i32, c_int), MessageCallback>,
}

impl CallbackRegistry {
  pub fn instance() -> &'static Self {
    static mut REGISTRY: OnceLock<CallbackRegistry> = OnceLock::new();
    unsafe { REGISTRY.get_or_init(|| CallbackRegistry { callbacks: DashMap::new() }) }
  }

  pub fn register(&self, sequence: i32, msg_id: i32, callback: MessageCallback) {
    self.callbacks.insert((sequence, msg_id), callback);
  }

  pub fn invoke_callback(&self, key: (i32, c_int), msg_ptr: *mut IBizMessageRust) {
    if let Some((_, callback)) = self.callbacks.remove(&key) {
      callback(msg_ptr);
    }
  }
}

static SEQUENCE_COUNTER: AtomicI32 = AtomicI32::new(0);

pub struct Connection {
  ptr: *mut IConnectionRust,
  sequence: i32,
}

impl Connection {
  pub fn new(config: *mut CConfigInterfaceRust) -> Result<Self> {
    let lib = crate::get_library()?;
    let sequence = SEQUENCE_COUNTER.fetch_add(1, Ordering::SeqCst);

    unsafe {
      let ptr = (lib.new_connection)(config);
      if ptr.is_null() {
        return Err(napi::Error::from_reason("Failed to create config instance"));
      }
      Ok(Connection { ptr, sequence })
    }
  }

  pub fn connect(&self, timeout: u32) -> Result<i32> {
    unsafe {
      let conn = &mut *self.ptr;
      let callback = Box::into_raw(create_callback(self.sequence));
      let result = conn.create_2_biz_msg(callback);
      Ok(conn.connect(timeout))
    }
  }

  // GetErrorMsg
  pub fn get_error_msg(&self, error_code: i32) -> Result<String> {
    unsafe {
      let conn = &*self.ptr;
      let msg = conn.get_error_msg(error_code);
      Ok(msg.to_string())
    }
  }

  pub fn close(&self) -> Result<i32> {
    unsafe {
      let conn = &mut *self.ptr;
      Ok(conn.close())
    }
  }

  pub fn send_biz_msg<F>(&self, message: *const IBizMessageRust, callback: F) -> Result<i32>
  where
    F: FnOnce(*mut IBizMessageRust) + Send + 'static,
  {
    unsafe {
      let conn = &*self.ptr;
      let msg_id = conn.send_biz_msg(message, 1);
      if msg_id > 0 {
        let registry = CallbackRegistry::instance();
        registry.register(self.sequence, msg_id, Box::new(callback));
        Ok(msg_id)
      } else {
        Err(napi::Error::from_reason("Failed to send message"))
      }
    }
  }

  // return Ok(0);
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

impl Drop for Connection {
  fn drop(&mut self) {
    unsafe {
      if !self.ptr.is_null() {
        println!("Connection Release");
        let conn = &*self.ptr;
        conn.release();
        self.ptr = std::ptr::null_mut();
      }
    }
  }
}
