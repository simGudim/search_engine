use crate::db::models::{NewUser, User, UpdateProfile};
use crate::db::{PgPool, Db};
use crate::conf::crypto::CryptoService;

use actix_web::{HttpResponse, Responder, get, post, web};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use askama::{Template};
use chrono::{Utc};
use failure::Error;
use std::path::PathBuf;
use std::env;
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "login.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let temp = HelloTemplate {
        name: "hello"
    };
    let s = temp.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/register")]
pub async fn register_link() -> Result<NamedFile> {
    let path: PathBuf = "./templates/register.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/register")]
pub async fn register(pool: web::Data<PgPool>, info: web::Form<NewUser>) -> impl Responder {
    let secret = env::var("SECRET_KEY")
        .expect("SECRET KEY must be set");
    let crypto = CryptoService::crypto_service(secret);
    let id_hash: String = crypto.hash_password(info.password.clone()).unwrap();
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    // Create insertion model
    let uuid_hash = Uuid::new_v4();
    let user = User {
        username: info.username.clone(),
        email: info.email.clone(),
        password_hash: id_hash,
        full_name: None,
        bio: None,
        image: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    };
    let row = Db::add_user(&user, conn).await.unwrap();
    // normal diesel operations
    HttpResponse::Ok().body(format!("Hello: {}", row))
}

#[post("/login")]
pub async fn login(info: web::Json<NewUser>) -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[post("/search")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}