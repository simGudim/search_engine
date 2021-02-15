mod forms;

use forms::{HelloTemplate, LoginForm, NewUser};
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
pub async fn register(pool: web::Data<PgPool>, info: web::Form<NewUser>) -> Result<NamedFile> {
    let secret = env::var("SECRET_KEY")
        .expect("SECRET KEY must be set");
    let crypto = CryptoService::crypto_service(secret);
    let id_hash: String = crypto.hash_password(info.password.clone()).unwrap();
    let conn = pool.get().expect("couldn't get db connection from pool");
    if let Some(user) = Db::get_user_by_username(info.username.clone(), &conn).await {
        if user.username == info.username {
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
    } else {
        let path: PathBuf = "./templates/register.html".parse().unwrap();
        Ok(NamedFile::open(path)?)
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, id: Identity, form: web::Form<LoginForm>) -> Result<NamedFile> {
    let mut verifier = Verifier::default();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = Db::get_user_by_username(form.username.clone(), &conn).await.unwrap();
    let is_valid = verifier
        .with_hash(user.password_hash)
        .with_password(form.password.clone())
        .with_secret_key(env::var("SECRET_KEY").expect("SECRET KEY must be set"))
        .verify()
        .unwrap();
    if is_valid {
        let path: PathBuf = "./templates/search.html".parse().unwrap();
        id.remember(form.username.to_owned());
        Ok(NamedFile::open(path)?)
    } else {
        let path: PathBuf = "./templates/login.html".parse().unwrap();
        Ok(NamedFile::open(path)?)
    }
}

#[get("logout")]
pub async fn logout(id: Identity) -> Result<NamedFile> {
    id.forget();                   
    let path: PathBuf = "./templates/login.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}



#[get("/search")]
pub async fn search(id: Identity) -> impl Responder {
    if let Some(id) = id.identity() {
        HttpResponse::Ok().body("hello")
    } else {
        HttpResponse::Ok().body("nope!")
    }
    
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}