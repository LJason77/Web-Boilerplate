#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::used_underscore_binding)]

use std::thread::spawn;

use flume::{unbounded, Receiver, Sender};
use mongodb::{Client, Database};
use rocket::{catchers, routes, Build, Rocket};

use api::{auth, index};
use logger::Logger;
use models::Results;

mod api;
mod catchers;
mod logger;
mod models;

/// 初始化 mongodb 数据库
async fn init_mongo(name: &str) -> Database {
    let database_url = dotenv::var("MONGO_URL").expect("MONGO_URL 未设置！");
    let client = Client::with_uri_str(&database_url).await.expect("连接数据库失败：");
    client.database(name)
}

#[allow(clippy::no_effect_underscore_binding)]
pub async fn rocket() -> Rocket<Build> {
    let db = init_mongo("web").await;

    let (tx, rx): (Sender<Logger>, Receiver<Logger>) = unbounded();
    let log_db = db.clone();
    spawn(move || Logger::receive(&log_db, rx));

    rocket::build()
        .manage(db)
        .manage(tx)
        .attach(Logger::default())
        // 登录和注册
        .mount("/", routes![auth::login])
        // 首页
        .mount("/", routes![index::auth_hallo, index::hallo, index::health_check])
        // 错误处理
        .register("/", catchers![catchers::not_found])
}
