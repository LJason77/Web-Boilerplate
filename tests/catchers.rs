use rocket::{http::Status, local::asynchronous::Client};

#[rocket::async_test]
async fn catchers() {
    let rocket = web_boilerplate::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    let response = client.get("/404").dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}
