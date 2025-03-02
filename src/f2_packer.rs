// packer.rs
use napi::{bindgen_prelude::Buffer, Result};
use napi_derive::napi;
use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_int, c_ulong};

use crate::f2_unpacker::{IF2UnPackerRust, UnPacker};
use crate::iknown::{IKnown, IKnownVTable};

pub(crate) trait IF2Packer: IKnown {
  unsafe fn set_buffer(&self, p_buf: *mut c_void, i_buf_size: c_int, i_data_len: c_int);
  unsafe fn begin_pack(&self);
  unsafe fn new_dataset(&self, sz_dataset_name: *const c_char, i_return_code: c_int) -> c_int;
  unsafe fn add_field(&self, sz_field_name: *const c_char, c_field_type: c_char, i_field_width: c_int, i_field_scale: c_int) -> c_int;
  unsafe fn add_str(&self, sz_value: *const c_char) -> c_int;
  unsafe fn add_int(&self, i_value: c_int) -> c_int;
  unsafe fn add_double(&self, d_value: f64) -> c_int;
  unsafe fn add_char(&self, c_value: c_char) -> c_int;
  unsafe fn add_raw(&self, p_raw: *const c_void, i_len: c_int) -> c_int;
  unsafe fn end_pack(&self);
  unsafe fn get_pack_buf(&self) -> *mut c_void;
  unsafe fn get_pack_len(&self) -> c_int;
  unsafe fn get_pack_buf_size(&self) -> c_int;
  unsafe fn get_version(&self) -> c_int;
  unsafe fn set_return_code(&self, dw_ret_code: c_ulong);
  unsafe fn unpack(&self) -> *mut IF2UnPackerRust;
  unsafe fn free_mem(&self, lp_buf: *mut c_void);
  unsafe fn clear_value(&self);
  unsafe fn begin_pack_ex(&self, sz_name: *const c_char);
  unsafe fn clear_data_set(&self);
}

#[repr(C)]
pub(crate) struct F2PackerVTable {
  pub iknown: IKnownVTable,
  set_buffer: extern "C" fn(*mut c_void, *mut c_void, c_int, c_int),
  begin_pack: extern "C" fn(*mut c_void),
  new_dataset: extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int,
  add_field: extern "C" fn(*mut c_void, *const c_char, c_char, c_int, c_int) -> c_int,
  add_str: extern "C" fn(*mut c_void, *const c_char) -> c_int,
  add_int: extern "C" fn(*mut c_void, c_int) -> c_int,
  add_double: extern "C" fn(*mut c_void, f64) -> c_int,
  add_char: extern "C" fn(*mut c_void, c_char) -> c_int,
  add_raw: extern "C" fn(*mut c_void, *const c_void, c_int) -> c_int,
  end_pack: extern "C" fn(*mut c_void),
  get_pack_buf: extern "C" fn(*mut c_void) -> *mut c_void,
  get_pack_len: extern "C" fn(*mut c_void) -> c_int,
  get_pack_buf_size: extern "C" fn(*mut c_void) -> c_int,
  get_version: extern "C" fn(*mut c_void) -> c_int,
  set_return_code: extern "C" fn(*mut c_void, c_ulong),
  unpack: extern "C" fn(*mut c_void) -> *mut IF2UnPackerRust,
  free_mem: extern "C" fn(*mut c_void, *mut c_void),
  clear_value: extern "C" fn(*mut c_void),
  begin_pack_ex: extern "C" fn(*mut c_void, *const c_char),
  clear_data_set: extern "C" fn(*mut c_void),
}

#[repr(C)]
pub(crate) struct IF2PackerRust {
  vtable: *const F2PackerVTable,
}

impl IKnown for IF2PackerRust {
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

impl IF2Packer for IF2PackerRust {
  unsafe fn set_buffer(&self, p_buf: *mut c_void, i_buf_size: c_int, i_data_len: c_int) {
    ((*self.vtable).set_buffer)(self as *const _ as *mut c_void, p_buf, i_buf_size, i_data_len)
  }
  unsafe fn begin_pack(&self) {
    ((*self.vtable).begin_pack)(self as *const _ as *mut c_void)
  }
  unsafe fn new_dataset(&self, sz_dataset_name: *const c_char, i_return_code: c_int) -> c_int {
    ((*self.vtable).new_dataset)(self as *const _ as *mut c_void, sz_dataset_name, i_return_code)
  }
  unsafe fn add_field(&self, sz_field_name: *const c_char, c_field_type: c_char, i_field_width: c_int, i_field_scale: c_int) -> c_int {
    ((*self.vtable).add_field)(
      self as *const _ as *mut c_void,
      sz_field_name,
      c_field_type,
      i_field_width,
      i_field_scale,
    )
  }
  unsafe fn add_str(&self, sz_value: *const c_char) -> c_int {
    ((*self.vtable).add_str)(self as *const _ as *mut c_void, sz_value)
  }
  unsafe fn add_int(&self, i_value: c_int) -> c_int {
    ((*self.vtable).add_int)(self as *const _ as *mut c_void, i_value)
  }
  unsafe fn add_double(&self, d_value: f64) -> c_int {
    ((*self.vtable).add_double)(self as *const _ as *mut c_void, d_value)
  }
  unsafe fn add_char(&self, c_value: c_char) -> c_int {
    ((*self.vtable).add_char)(self as *const _ as *mut c_void, c_value)
  }
  unsafe fn add_raw(&self, p_raw: *const c_void, i_len: c_int) -> c_int {
    ((*self.vtable).add_raw)(self as *const _ as *mut c_void, p_raw, i_len)
  }
  unsafe fn end_pack(&self) {
    ((*self.vtable).end_pack)(self as *const _ as *mut c_void)
  }
  unsafe fn get_pack_buf(&self) -> *mut c_void {
    ((*self.vtable).get_pack_buf)(self as *const _ as *mut c_void)
  }
  unsafe fn get_pack_len(&self) -> c_int {
    ((*self.vtable).get_pack_len)(self as *const _ as *mut c_void)
  }
  unsafe fn get_pack_buf_size(&self) -> c_int {
    ((*self.vtable).get_pack_buf_size)(self as *const _ as *mut c_void)
  }
  unsafe fn get_version(&self) -> c_int {
    ((*self.vtable).get_version)(self as *const _ as *mut c_void)
  }
  unsafe fn set_return_code(&self, dw_ret_code: c_ulong) {
    ((*self.vtable).set_return_code)(self as *const _ as *mut c_void, dw_ret_code)
  }
  unsafe fn unpack(&self) -> *mut IF2UnPackerRust {
    ((*self.vtable).unpack)(self as *const _ as *mut c_void)
  }
  unsafe fn free_mem(&self, lp_buf: *mut c_void) {
    ((*self.vtable).free_mem)(self as *const _ as *mut c_void, lp_buf)
  }
  unsafe fn clear_value(&self) {
    ((*self.vtable).clear_value)(self as *const _ as *mut c_void)
  }
  unsafe fn begin_pack_ex(&self, sz_name: *const c_char) {
    ((*self.vtable).begin_pack_ex)(self as *const _ as *mut c_void, sz_name)
  }
  unsafe fn clear_data_set(&self) {
    ((*self.vtable).clear_data_set)(self as *const _ as *mut c_void)
  }
}

#[napi]
pub struct Packer {
  packer_ptr: *mut IF2PackerRust,
}
impl Packer {
  pub fn new(version: i32) -> Result<Self> {
    let lib = crate::get_library()?;

    unsafe {
      let packer_ptr = (lib.new_packer)(version);
      if packer_ptr.is_null() {
        return Err(napi::Error::from_reason("Failed to create packer instance"));
      }

      Ok(Packer { packer_ptr })
    }
  }

  fn check_ptr(&self) -> Result<&IF2PackerRust> {
    unsafe {
      if self.packer_ptr.is_null() {
        return Err(napi::Error::from_reason("Packer is not initialized"));
      }
      Ok(&*self.packer_ptr)
    }
  }

  fn to_c_string(s: String, field: &str) -> Result<CString> {
    CString::new(s).map_err(|e| napi::Error::from_reason(format!("Invalid {}: {}", field, e)))
  }
}

#[napi]
impl Packer {
  /**
   * 打包器初始化(使用调用者的缓存区)
   * 第一次使用打包器时，可先使用本方法设置好缓冲区(数据长度被置为iDataLen)
   * @param  char * pBuf  缓冲区地址
   * @param  int iBufSize  缓冲区空间
   * @param  int iDataLen  已有数据长度，新增数据加在已有数据之后（只对V1.0格式的包有效）
   */
  #[napi]
  pub fn set_buffer(&self, p_buf: Buffer, i_data_len: Option<i32>) -> Result<()> {
    let data_len = i_data_len.unwrap_or(0);
    unsafe {
      let p_buf_ptr = p_buf.as_ptr() as *mut c_void;
      let i_buf_size = p_buf.len() as i32;
      self.check_ptr()?.set_buffer(p_buf_ptr, i_buf_size, data_len);
    }
    Ok(())
  }
  /**
   * 复位，重新开始打另一个包(字段数与记录数置为0行0例)
   * 功能：开始打包，把包长度清零(重复使用已有的缓存区空间)
   * @return 无
   */
  #[napi]
  pub fn begin_pack(&self) -> Result<()> {
    unsafe {
      self.check_ptr()?.begin_pack();
    }
    Ok(())
  }
  /**
   * 开始打一个结果集
   * 在打单结果集的包时，可以不调用本方法,均取默认值
   * @param const char *szDatasetName 0x20版打包需要指明结果集名字
   * @param int iReturnCode           0x20版打包需要为每个结果集指明返回值
   */
  #[napi]
  pub fn new_dataset(&self, sz_dataset_name: String, i_return_code: i32) -> Result<i32> {
    let sz_dataset_name = Self::to_c_string(sz_dataset_name, "sz_dataset_name")?;
    let result = unsafe { self.check_ptr()?.new_dataset(sz_dataset_name.as_ptr(), i_return_code) };
    Ok(result)
  }
  /**
   * 功能：向包添加字段
   *
   * 有执行次序要求:在 NewDataset()或Reset(),SetBuffer()之后,逐个字段按顺序添加;
   *
   * @param szFieldName：字段名
   * @param cFieldType ：字段类型:I整数，D浮点数，C字符，S字符串，R任意二进制数据
   * @param iFieldWidth ：字段宽度（所占最大字节数）
   * @param iFieldScale ：字段精度,即cFieldType='D'时的小数位数(缺省为4位小数)
   * @return 负数表示失败，否则为目前包的长度
   */
  #[napi]
  pub fn add_field(&self, sz_field_name: String, c_field_type: i8, i_field_width: i32, i_field_scale: i32) -> Result<i32> {
    use std::os::raw::{c_char, c_int};

    let sz_field_name = Self::to_c_string(sz_field_name, "sz_field_name")?;
    // 将 char 转换为 c_char
    let c_field_type = c_field_type as c_char;

    let result = unsafe {
      self
        .check_ptr()?
        .add_field(sz_field_name.as_ptr(), c_field_type, i_field_width as c_int, i_field_scale as c_int)
    };

    Ok(result)
  }
  /**
   * 功能：向包添加字符串数据
   * 有执行次序要求:必须在所有字段增加完之后,逐个字段按顺序添加;
   * @param       szValue：字符串数据
   * @return 负数表示失败，否则为目前包的长度
   */
  #[napi]
  pub fn add_str(&self, sz_value: String) -> Result<i32> {
    let sz_value = Self::to_c_string(sz_value, "sz_value")?;
    let result = unsafe { self.check_ptr()?.add_str(sz_value.as_ptr()) };
    Ok(result)
  }

  /**
   * 功能：向包添加整数数据
   * @param       iValue：整数数据
   * @return 负数表示失败，否则为目前包的长度
   */
  #[napi]
  pub fn add_int(&self, i_value: i32) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.add_int(i_value) };
    Ok(result)
  }
  /**
   * 功能：向包添加浮点数据
   * @param       fValue：浮点数据
   * @return 负数表示失败，否则为目前包的长度
   */
  #[napi]
  pub fn add_double(&self, d_value: f64) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.add_double(d_value) };
    Ok(result)
  }
  /**
   * 功能：向包添加一个字符
   * @param		 cValue：字符
   * @return 负数表示失败，否则为目前包的长度
   */
  #[napi]
  pub fn add_char(&self, c_value: i8) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.add_char(c_value as c_char) };
    Ok(result)
  }
  /**
   * 功能：向包添加一个大对象
   * @param	void * lpBuff 数据区
   * @param	int iLen  数据长度
   * @return 负数表示失败，否则为目前包的长度
   */
  #[napi]
  pub fn add_raw(&self, p_raw: Buffer) -> Result<i32> {
    let p_raw_ptr = p_raw.as_ptr();
    let i_len = p_raw.len() as i32; // 直接使用 Buffer 的 len() 方法
    let result = unsafe { self.check_ptr()?.add_raw(p_raw_ptr as *const c_void, i_len) };
    Ok(result)
  }
  ///结束打包

  #[napi]
  pub fn end_pack(&self) -> Result<()> {
    unsafe {
      self.check_ptr()?.end_pack();
    }
    Ok(())
  }
  /**
   * 功能：取打包结果指针
   * @return 打包结果指针
   */
  #[napi]
  pub fn get_pack_buf(&self) -> Result<Buffer> {
    let result = unsafe {
      let pack_buf = self.check_ptr()?.get_pack_buf();
      let pack_len = self.check_ptr()?.get_pack_len();
      Buffer::from(std::slice::from_raw_parts(pack_buf as *const u8, pack_len as usize))
    };
    Ok(result)
  }
  /**
   * 功能：取打包结果长度
   * @return 打包结果长度
   */
  #[napi]
  pub fn get_pack_len(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_pack_len() };
    Ok(result)
  }
  /**
   * 功能：取打包结果缓冲区大小
   * @return 打包结果缓冲区大小
   */
  #[napi]
  pub fn get_pack_buf_size(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_pack_buf_size() };
    Ok(result)
  }
  /**
   * 功能：取打包格式版本
   * @return 版本
   */
  #[napi]
  pub fn get_version(&self) -> Result<i32> {
    let result = unsafe { self.check_ptr()?.get_version() };
    Ok(result)
  }
  /**
   * 设置结果集的返回码(0x20版以上要求)，错误结果集需要设置
   * 返回码取缺省值0，则不设置，如果设置，则必须在EndPack()之前调用
   * @return 版本
   */
  #[napi]
  pub fn set_return_code(&self, dw_ret_code: u32) -> Result<()> {
    unsafe {
      let dw_ret_code = dw_ret_code as c_ulong;
      self.check_ptr()?.set_return_code(dw_ret_code);
    }
    Ok(())
  }

  /**
   * 直接返回当前打包结果的解包接口,必须在EndPack()之后才能调用,在打包器释放时相应的解包器实例也释放
   * @return 解包器接口，此解包接口不能调用 destroy()来释放
   */
  #[napi]
  pub fn unpack(&self) -> Result<UnPacker> {
    let raw_ptr = unsafe { self.check_ptr()?.unpack() };
    if raw_ptr.is_null() {
      return Err(napi::Error::from_reason("Failed to unpack, please call end_pack() first"));
    }
    Ok(UnPacker::from_ptr(raw_ptr))
  }

  #[napi]
  pub fn free_mem(&self) -> Result<()> {
    unsafe {
      let lp_buf = self.check_ptr()?.get_pack_buf();
      self.check_ptr()?.free_mem(lp_buf);
    }
    Ok(())
  }

  #[napi]
  pub fn clear_value(&self) -> Result<()> {
    unsafe {
      self.check_ptr()?.clear_value();
    }
    Ok(())
  }
  //20110302 xuxp 增加一个接口函数，用来传递第一个结果集的名字
  ///
  /**
   * 复位，重新开始打另一个包(字段数与记录数置为0行0例)
   * 功能：开始打包，把包长度清零(重复使用已有的缓存区空间)
   * @return 无
   */
  #[napi]
  pub fn begin_pack_ex(&self, sz_name: String) -> Result<()> {
    let sz_name = Self::to_c_string(sz_name, "sz_name")?;
    unsafe {
      self.check_ptr()?.begin_pack_ex(sz_name.as_ptr());
    }
    Ok(())
  }
  /// 20110324 dongpf 增加一个接口函数，用来复位当前结果集
  /**
   * 复位当前结果集(字段数与记录数置为0行0例)，不影响其他结果集
   * 功能：复位当前结果集
   * @return 无
   */
  #[napi]
  pub fn clear_data_set(&self) -> Result<()> {
    unsafe {
      self.check_ptr()?.clear_data_set();
    }
    Ok(())
  }
}

impl Drop for Packer {
  fn drop(&mut self) {
    unsafe {
      if !self.packer_ptr.is_null() {
        let ptr = &*self.packer_ptr;
        ptr.release();
        self.packer_ptr = std::ptr::null_mut();
      }
    }
  }
}

unsafe impl Send for Packer {}
unsafe impl Sync for Packer {}
