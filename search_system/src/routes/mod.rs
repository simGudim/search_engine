mod forms;

use analyzer;
use forms::{HelloTemplate, LoginForm, NewUser, DirForm};
use crate::db::models::{User};
use crate::db::{PgPool, Db};
use crate::conf::crypto::CryptoService;
use crate::mongo::{Mongo, MongoConn, MongoPool};

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


//IDEAs
//Make query AND OR and proximit searcg
//make preview window
//make a confog file for analyzer and options



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
pub async fn login_link() -> Result<NamedFile> {
    let path: PathBuf = "./templates/login.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}


//make this multithreaded and make this into one call
#[post("/pipe")]
pub async fn pipe(pool:web::Data<MongoPool>, form: web::Form<DirForm>, id: Identity) -> impl Responder {
    if id.identity().is_some() {
        let conn = pool.get().expect("couldn't get db connection from pool");
        let documents = analyzer::read_files_from_dir(form.dir.as_str());
        let mut tokens = vec![];
        for i in documents {
            tokens.push(analyzer::create_tokens_list(&i));
        }
        let index = analyzer::create_index(tokens);

        for (key, value) in index.into_iter() {
            Mongo::add_word(&conn, key, value).await;
        }
        HttpResponse::Ok().body("Index is running")
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>,form: web::Form<LoginForm>, id: Identity) -> Result<NamedFile> {
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
        let path: PathBuf = "./templates/search.html".parse().unwrap();
        id.remember(user.id.to_string());
        Ok(NamedFile::open(path)?)
    } else {
        let path: PathBuf = "./templates/login.html".parse().unwrap();
        Ok(NamedFile::open(path)?)
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

#[get("/search")]
pub async fn search(id: Identity) -> impl Responder{
    if id.identity().is_some() {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(forms::Search.render().unwrap())
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
    
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}