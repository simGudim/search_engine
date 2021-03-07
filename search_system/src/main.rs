mod conf;
mod db;
mod routes;
mod mongo;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

extern crate actix_web;
extern crate argonautica;

use crate::conf::Config;
use crate::db::Db;
use crate::mongo::Mongo;

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
            .service(routes::register_get)
            .service(routes::register_post)
            .service(routes::login_get)
            .service(routes::login_post)
            .service(routes::submit_index_get)
            .service(routes::submit_index_post)
            .service(routes::search_get)
            .service(routes::logout)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
