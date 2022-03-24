use rocket::{http::Status, local::asynchronous::Client};

#[rocket::async_test]
async fn index() {
    let rocket = web_boilerplate::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    // 健康检查
    let response = client.get("/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);

    // hallo
    let response = client.get("/hallo").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
