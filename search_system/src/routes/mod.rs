mod forms;
pub mod login;
pub mod indexing;
pub mod search;

use forms::{HelloTemplate};
use actix_web::{HttpResponse, Responder};
use actix_web::{HttpRequest, Result};
use askama::{Template};



pub async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let temp = HelloTemplate {
        name: "hello"
    };
    let s = temp.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}