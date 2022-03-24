use serde::{Deserialize, Serialize};

use super::deserialize_oid;

/// 用户
///
/// Option 为可选
#[derive(Serialize, Deserialize)]
pub struct User {
    /// id
    #[serde(deserialize_with = "deserialize_oid")]
    pub _id: String,
    /// 用户名
    pub name: String,
    /// 密码
    pub password: String,
}
