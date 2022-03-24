//! 用户验证
//!
//! # 登录
//!
//! api： /login
//! - 方法： post
//! - 数据： [Login](../../models/struct.Login.html)
//! - 返回：`{ "token": token }`

use bson::doc;
use mongodb::Database;
use rocket::{
    http::Status,
    post,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
    State,
};

use crate::{
    models::{auth::Login, user::User, Claims},
    Results,
};

/// 登录用户，返回一个带有 token 的返回
///
/// 找不到用户或密码错误时返回 UNAUTHORIZED
#[post("/login", data = "<login>", format = "application/json")]
pub async fn login(db: &State<Database>, login: Json<Login>) -> Custom<Value> {
    let mut result = Results::<Value>::default();

    let collection = db.collection::<User>("users");
    let option = collection
        .find_one(
            doc! {
                "name": &login.name
            },
            None,
        )
        .await
        .unwrap();

    if let Some(user) = option {
        if user.password == login.password {
            result.data = Some(json!({ "token": Claims::encode(&user) }));
        } else {
            result.message = Some("密码错误");
            result.code = Some(Status::Unauthorized.code);
        }
    } else {
        result.message = Some("用户不存在");
        result.code = Some(Status::Unauthorized.code);
    }

    Custom(Status::new(result.code.unwrap_or(200)), json!(result))
}
