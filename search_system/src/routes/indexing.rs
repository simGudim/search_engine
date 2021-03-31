

use analyzer;
use crate::routes::forms;
use crate::mongo::{Mongo, MongoConn, MongoPool};
use crate::readers;

use actix_web::{HttpResponse, Responder, get, post, web};
use actix_web::{HttpRequest, Result};
use actix_identity::{Identity};
use askama::{Template};
use chrono::{Utc};

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
pub async fn submit_index_post(pool:web::Data<MongoPool>, form: web::Form<forms::DirForm>, id: Identity) -> impl Responder {
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
