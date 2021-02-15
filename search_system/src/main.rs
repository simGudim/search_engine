mod conf;
mod db;
mod routes;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate actix_web;
extern crate argonautica;

use crate::conf::Config;
use crate::db::Db;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use tracing::{info, instrument};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::from_env()
        .expect("Server configuration");

    let db = Db::establish_connection().await;
    
    info!("Starting server at http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32]) 
                      .name("auth-cookie")
                      .secure(false)))
            .data(db.pool.clone())
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route("/", web::get().to(routes::index))
            .service(routes::register)
            .service(routes::register_link)
            .service(routes::login)
            .service(routes::search)
            .service(routes::logout)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
