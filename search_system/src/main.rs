mod conf;
mod db;
mod routes;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

use crate::conf::Config;
use crate::db::Db;
use crate::routes::{echo, login, manual_hello, index};

use actix_files as fs;
use actix_web::{get, post, web, App, HttpServer};
use actix_web::middleware::Logger;
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
            .data(db.pool.clone())
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route("/", web::get().to(index))
            .service(echo)
            .service(login)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
