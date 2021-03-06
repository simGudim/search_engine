pub mod models;
pub mod schema;


use models::{User};

use diesel::{Connection, ExpressionMethods,PgConnection, 
    QueryDsl, RunQueryDsl, insert_into};
use std::env;
use tracing::{info, instrument};
use failure::Error;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager};

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

    pub async fn add_user(user: &models::User, conn: PgPooledConnection) -> Result<usize, Error> {
        use self::schema::users::dsl::*;

        let row_inserted = insert_into(users)
            .values(user)
            .returning(schema::users::id)
            .execute(&conn)
            .expect("Error inserting person");
        Ok(row_inserted)
    }

    pub async fn get_user_by_username(user_name: &String, conn: &PgPooledConnection) -> Option<User> {
        use self::schema::users::dsl::*;
        let mut items = users
            .filter(username.eq(user_name))
            .load::<models::User>(conn)
            .expect("Error loading person");
        items.pop()
    }

}
