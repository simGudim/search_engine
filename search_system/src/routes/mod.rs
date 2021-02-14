use crate::db::models::{NewUser, User, UpdateProfile};
use crate::db::schema::users;

use actix_web::{HttpResponse, Responder, get, post, web};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use askama::{Template};
use std::path::PathBuf;
use tracing::{info, instrument};

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

#[post("/register")]
pub async fn register(info: web::Json<NewUser>) -> impl Responder {
    HttpResponse::Ok().body("Login")
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