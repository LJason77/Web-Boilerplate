use rocket::{
    catch,
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Value},
    Request,
};

use crate::Results;

#[catch(404)]
pub fn not_found(status: Status, req: &Request) -> Custom<Value> {
    let mut result = Results::<&str>::default();
    println!("{}：\nIP：{:?}\nurl：{:?}\n", status, &req.client_ip(), req.uri().path().as_str());
    result.message = Some("路由未找到");
    result.code = Some(status.code);
    Custom(status, json!(result))
}
