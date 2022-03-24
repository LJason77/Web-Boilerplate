use dotenv::dotenv;

#[rocket::main]
async fn main() {
    dotenv().ok();

    let rocket = web_boilerplate::rocket().await;
    if let Err(err) = rocket.launch().await {
        println!("Rocket 启动错误: {:?}", err);
    }
}
