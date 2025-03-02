use napi_derive::napi;

// 文件更新相关字段常量
#[napi]
pub const PACKER_INT_FILE_LIST: &str = "file_list"; // 文件列表
#[napi]
pub const PACKER_INT_FILE_ID: &str = "file_id"; // 文件id
#[napi]
pub const PACKER_INT_FILE_SIZE: &str = "file_size"; // 文件大小
#[napi]
pub const PACKER_INT_TIME: &str = "time"; // 文件时间
#[napi]
pub const PACKER_STRING_FILE_NAME: &str = "file_name"; // 文件名
#[napi]
pub const PACKER_STRING_FILE_PATH: &str = "file_path"; // 文件路径
#[napi]
pub const PACKER_STRING_LOCAL_PATH: &str = "local_path"; // 本地文件路径
#[napi]
pub const PACKER_STRING_MD5_CODE: &str = "md5_code"; // 文件md5值
#[napi]
pub const PACKER_STRING_FILE_FLAG: &str = "file_flag"; // 文件标识

// 业务消息类型
#[napi]
pub const REQUEST_PACKET: i32 = 0; // 请求
#[napi]
pub const ANSWER_PACKET: i32 = 1; // 应答

pub const PACKER_VERSION_V2: i32 = 0x20; // 打包器版本号
