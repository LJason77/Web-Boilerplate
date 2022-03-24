use bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};

pub use {auth::Claims, user::User};

pub mod auth;
pub mod user;

/// 对象结果
///
/// 返回给前端的对象结果
#[derive(Default, Serialize, Deserialize)]
pub struct Results<'a, T> {
    /// 状态码，只在错误时出现
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 信息，只在错误时出现
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
}

/// 将 `ObjectId` 反序列化为字符串
///
/// # Errors
/// 反序列化时，如果 `ObjectId` 不是字符串，则会抛出异常
#[allow(clippy::unnecessary_wraps)]
pub fn deserialize_oid<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let id: ObjectId = Deserialize::deserialize(d).expect("反序列化 ObjectId 失败");
    Ok(id.to_hex())
}
