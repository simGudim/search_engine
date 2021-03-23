mod forms;

use analyzer;
use forms::{HelloTemplate, LoginForm, NewUser, DirForm, QueryForm};
use crate::db::models::{User};
use crate::db::{PgPool, Db};
use crate::conf::crypto::CryptoService;
use crate::mongo::{Mongo, MongoConn, MongoPool};
use crate::readers;

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
use tracing::{info, debug, instrument};


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
pub async fn register_get() -> Result<NamedFile> {
    let path: PathBuf = "./templates/register.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/register")]
pub async fn register_post(pool: web::Data<PgPool>, info: web::Form<NewUser>) -> Result<NamedFile> {
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
pub async fn login_post(pool: web::Data<PgPool>,form: web::Form<LoginForm>, id: Identity) -> impl Responder {
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

#[get("/list_index")]
pub async fn list_index(pool:web::Data<MongoPool>, id: Identity)-> impl Responder {
    if id.identity().is_some() {
        let conn = pool.get().expect("couldn't get db connection from pool");
        let index = Mongo::list_index(&conn).await.unwrap();
        info!("{:?}", index);
        HttpResponse::Found().header("Location", "/login").finish()
    } else {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(forms::Something.render().unwrap())
    }              
}

#[get("/search")]
pub async fn search_get(id: Identity) -> impl Responder {
    if id.identity().is_some() {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(forms::SearchGet.render().unwrap())
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
    
}

#[post("/search")]
pub async fn search_post(pool:web::Data<MongoPool>, form: web::Form<QueryForm>,id: Identity) -> impl Responder{
    if id.identity().is_some() {
        let conn = pool.get().expect("couldn't get db connection from pool");
        let query_type: &str;
        if form.query_terms.contains("/AND") {
            query_type = "AND";
        } else if form.query_terms.contains("/OR") {
            query_type = "OR";
        } else {
            query_type = "";
        }
        if query_type == "AND" {
            let query_words = analyzer::create_tokens_list(&form.query_terms.replace("/AND", ""));
            let query_len = query_words.len();
            let mut result = Mongo::query_words(&conn, query_words).await.unwrap();
            if result.len() == query_len && query_len > 1{
                result.sort_by(|a, b| b.doc_len.cmp(&a.doc_len));
                let mut counter = 1;
                let mut post1 = &result[0].docs;
                let mut post2 = &result.last().unwrap().docs;
                let mut intersection = analyzer::intersect_list(post1, post2);
                while counter < result.len() - 1 {
                    let current_list = &result[counter].docs;
                    println!("{:?}", counter);
                    if intersection.len() > 0 {
                        intersection = analyzer::intersect_list(current_list, &intersection);
                    }
                    println!("{:?}", intersection);
                    counter += 1;
                }
                HttpResponse::Ok().body(format!("{:?}", intersection))
            } else if result.len() == query_len && query_len == 1 {
                HttpResponse::Ok().body(format!("{:?}", result))
            } else {
                HttpResponse::Ok().body("/AND query came out empty")
            }
        } else {
            let query_words = analyzer::create_tokens_list(&form.query_terms);
            let result = Mongo::query_words(&conn, query_words).await.unwrap();
            HttpResponse::Ok().body(format!("{:?}", result))
        }
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
    
}

#[get("/submit_index")]
pub async fn submit_index_get(id: Identity) -> impl Responder{
    if id.identity().is_some() {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(forms::SubmitIndex.render().unwrap())
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
    
}

//make this multithreaded and make this into one call
#[post("/submit_index")]
pub async fn submit_index_post(pool:web::Data<MongoPool>, form: web::Form<DirForm>, id: Identity) -> impl Responder {
    if id.identity().is_some() {
        let conn = pool.get().expect("couldn't get db connection from pool");
        let documents = readers::read_files_from_dir(form.dir.as_str());
        let mut tokens = vec![];
        for i in documents {
            tokens.push(analyzer::create_tokens_list(&i));
        }
        let index = analyzer::create_index(tokens);

        for (key, value) in index.into_iter() {
            Mongo::add_word(&conn, key, value).await;
        }
        HttpResponse::Found().header("Location", "/search").finish()
    } else {
        HttpResponse::Found().header("Location", "/login").finish()
    }
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}