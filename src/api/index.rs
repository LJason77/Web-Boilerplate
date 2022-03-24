use mongodb::Database;
use rocket::{
    get,
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Value},
    State,
};

use crate::{models::Claims, Results};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
#[get("/hallo")]
pub fn auth_hallo(_claims: Claims) -> Custom<Value> {
    let result = Results::<'_, &str> { data: Some("auth_hallo"), ..Results::default() };

    Custom(Status::Ok, json!(result))
}

#[must_use]
#[get("/hallo", rank = 2)]
pub fn hallo() -> Custom<Value> {
    let result = Results::<'_, &str> { data: Some("hallo"), ..Results::default() };

    Custom(Status::Ok, json!(result))
}

/// 健康检查
#[get("/health")]
pub async fn health_check(db: &State<Database>) -> Custom<Value> {
    let result = Results::<Vec<String>> {
        code: None,
        message: Some("服务器运行正常"),
        data: Some(db.list_collection_names(None).await.unwrap()),
    };
    Custom(Status::Ok, json!(result))
}
