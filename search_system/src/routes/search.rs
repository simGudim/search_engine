
use crate::routes::forms;

use analyzer;
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
pub async fn search_post(pool:web::Data<MongoPool>, form: web::Form<forms::QueryForm>,id: Identity) -> impl Responder{
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
                    if intersection.len() > 0 {
                        intersection = analyzer::intersect_list(current_list, &intersection);
                    }
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