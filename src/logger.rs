use chrono::Local;
use flume::{Receiver, Sender};
use futures::executor::block_on;
use mongodb::Database;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Logger {
    /// 用户名
    pub user: String,
    /// 发起地址
    pub ip: String,
    /// 请求路径
    pub uri: String,
    /// 请求方法
    pub method: String,
    /// 状态码
    pub status: u16,
    /// 请求时间
    pub time: i64,
}

impl Logger {
    pub fn send(self, req: &Request) {
        let tx = req.rocket().state::<Sender<Logger>>().expect("Sender not found");
        if let Err(err) = tx.send(self) {
            println!("Err: {:?}", err.to_string());
        }
    }

    pub fn receive(database: &Database, rx: Receiver<Self>) {
        let collection = database.collection::<Logger>("logs");
        block_on(async {
            for logger in rx {
                collection.insert_one(&logger, None).await.ok();
            }
        });
    }
}

#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info { name: "Logger", kind: Kind::Response }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let user = request.local_cache(String::new).to_string();
        let ip = match request.client_ip() {
            None => "未知".to_string(),
            Some(ip) => ip.to_string(),
        };
        let logger = Logger {
            user,
            ip,
            uri: request.uri().to_string(),
            method: request.method().to_string(),
            status: response.status().code,
            time: Local::now().timestamp(),
        };
        logger.send(request);
    }
}
