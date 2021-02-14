pub mod models;
pub mod schema;


use crate::conf::crypto::CryptoService;

use diesel::{Connection, ExpressionMethods, OptionalExtension, PgConnection, 
    QueryDsl, RunQueryDsl, insert_into};
use std::env;
use tracing::{info, instrument};
use failure::Error;
use std::thread;
use r2d2::ManageConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool
}

impl Db {
    #[instrument]
    pub async fn establish_connection() -> Self {
        info!("Creating db connection pool");
        let database_url = env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager).unwrap();
        let db = Self {
            pool
        };
        db    
    }

}

