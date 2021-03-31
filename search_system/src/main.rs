mod conf;
mod db;
mod routes;
mod mongo;
mod readers;


#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

extern crate actix_web;
extern crate argonautica;

use crate::conf::Config;
use crate::db::Db;
use crate::mongo::Mongo;
use crate::routes::{login, indexing, search};

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use tracing::{info};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::from_env()
        .expect("Server configuration");

    let db = Db::establish_connection().await;

    let mongo_pool = Mongo::establish_mongo_conn().await;
    
    info!("Starting server at http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32]) 
                      .name("auth-cookie")
                      .secure(false)))
            .data(db.pool.clone())
            .data(mongo_pool.clone())
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route("/", web::get().to(routes::index))
            .service(login::register_get)
            .service(login::register_post)
            .service(login::login_get)
            .service(login::login_post)
            .service(login::logout)
            .service(indexing::submit_index_get)
            .service(indexing::submit_index_post)
            .service(search::search_get)
            .service(search::search_post)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
