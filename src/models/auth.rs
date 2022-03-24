use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{outcome::Outcome, request, request::FromRequest, Request};
use serde::{Deserialize, Serialize};

use super::User;

/// 登录数据
#[derive(Serialize, Deserialize)]
pub struct Login {
    /// 用户名
    pub name: String,
    /// 密码
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    /// 名称
    pub name: String,
    /// 发布时间
    pub iat: i64,
    /// 到期时间
    pub exp: i64,
}

impl Claims {
    /// 过期时间
    ///
    /// 9 小时
    const JWT_LIFETIME: i64 = 60 * 60 * 9;

    /// 编码 token
    #[must_use]
    pub fn encode(user: &User) -> String {
        let id = user._id.clone();
        let name = user.name.clone();
        let iat = Local::now().timestamp();
        let exp = iat + Self::JWT_LIFETIME;
        let claims = Self { id, name, iat, exp };

        let private_key = include_bytes!("private_ecdsa_key.pem");
        let encrypted = encode(
            &Header::new(Algorithm::ES384),
            &claims,
            &EncodingKey::from_ec_pem(private_key).expect(
                "从 .pem 文件加载 ECDSA 密钥，如果该密钥不是有效的私有 EC 密钥，则会出现此错误",
            ),
        );
        encrypted.expect("生成 token 失败")
    }

    /// 解码 token
    ///
    /// # Errors
    /// 当 token 无效时，返回错误
    pub fn decode(token: &str) -> Result<Self, &'static str> {
        let public_key = include_bytes!("public_ecdsa_key.pem");
        let decode_token = decode::<Self>(
            token,
            &DecodingKey::from_ec_pem(public_key)
                .expect("从 .pem 文件加载 ECDSA 公钥，如果该公钥不是有效的公钥，则会出现此错误"),
            &Validation::new(Algorithm::ES384),
        );
        if let Ok(token_data) = decode_token {
            Ok(token_data.claims)
        } else {
            Err("Authorization 错误")
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let encode_token = if let Some(bearer) = request.headers().get_one("Authorization") {
            bearer
        } else {
            return Outcome::Forward(());
        };
        let result = Self::decode(encode_token);
        match result {
            Ok(claims) => Outcome::Success(claims),
            Err(_) => Outcome::Forward(()),
        }
    }
}
