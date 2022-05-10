use std::collections::HashMap;

use rocket::{
    http::{Header, Status},
    local::asynchronous::Client,
    serde::json::Value,
};

#[rocket::async_test]
async fn auth() {
    let rocket = web_boilerplate::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    // 登录
    let login = HashMap::from([("name", "admin"), ("password", "admin")]);
    let response = client.post("/login").json(&login).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let token = response
        .into_json::<Value>()
        .await
        .unwrap()
        .get("data")
        .unwrap()
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // hallo(认证)
    let response =
        client.get("/hallo").header(Header::new("Authorization", token)).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
