use rocket::{http::Status, local::asynchronous::Client};
use web_boilerplate::models::auth::Login;

#[rocket::async_test]
async fn auth() {
    let rocket = web_boilerplate::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    // 登录
    let login = Login {
        name: "admin".to_string(),
        password: "c403b04d4617d99596164dbac8319d11".to_string(),
    };
    let response = client.post("/login").json(&login).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
