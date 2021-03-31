

use crate::routes::forms;
use crate::db::models::{User};
use crate::db::{PgPool, Db};
use crate::conf::crypto::CryptoService;


use actix_web::{HttpResponse, Responder, get, post, web};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use actix_identity::{Identity};
use argonautica::Verifier;
use askama::{Template};
use chrono::{Utc};
use std::path::PathBuf;
use std::env;
use uuid::Uuid;

#[get("/register")]
pub async fn register_get() -> Result<NamedFile> {
    let path: PathBuf = "./templates/register.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/register")]
pub async fn register_post(pool: web::Data<PgPool>, info: web::Form<forms::NewUser>) -> Result<NamedFile> {
    let secret = env::var("SECRET_KEY")
        .expect("SECRET KEY must be set");
    let crypto = CryptoService::crypto_service(secret);
    let id_hash: String = crypto.hash_password(info.password.clone()).unwrap();
    let conn = pool.get().expect("couldn't get db connection from pool");

    if let Some(_user) = Db::get_user_by_username(&info.username, &conn).await {
        let path: PathBuf = "./templates/register.html".parse().unwrap();
        Ok(NamedFile::open(path)?)
    } else {
        let uuid_hash = Uuid::new_v4();
        let user = User {
            id: uuid_hash,
            username: info.username.clone(),
            email: info.email.clone(),
            password_hash: id_hash,
            full_name: None,
            bio: None,
            image: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };
        Db::add_user(&user, conn).await.unwrap();
        let path: PathBuf = "./templates/login.html".parse().unwrap();
        Ok(NamedFile::open(path)?)
    }
}

#[get("/login")]
pub async fn login_get() -> impl Responder {
    // let path: PathBuf = "./templates/login.html".parse().unwrap();
    // Ok(NamedFile::open(path)?)
    HttpResponse::Ok()
            .content_type("text/html")
            .body(forms::LoginGet.render().unwrap())
}

#[post("/login")]
pub async fn login_post(pool: web::Data<PgPool>,form: web::Form<forms::LoginForm>, id: Identity) -> impl Responder {
    let mut verifier = Verifier::default();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = Db::get_user_by_username(&form.username, &conn).await.unwrap();
    let is_valid = verifier
        .with_hash(user.password_hash)
        .with_password(form.password.clone())
        .with_secret_key(env::var("SECRET_KEY").expect("SECRET KEY must be set"))
        .verify()
        .unwrap();
    if is_valid {
        id.remember(user.id.to_string());
        HttpResponse::Found().header("Location", "/submit_index").finish()
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
}

#[get("/logout")]
pub async fn logout(id: Identity)-> impl Responder {
    if id.identity().is_some() {
        id.forget();
        HttpResponse::Found().header("Location", "/login").finish()
    } else {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(forms::Something.render().unwrap())
    }              
}