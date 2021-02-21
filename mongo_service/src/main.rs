use bson::{bson, doc};
use chrono::offset::Utc;
use mongodb::Error as MongoEror;
use r2d2::Pool;
use mongodb::db::{Database, ThreadedDatabase};
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use serde_derive::{Deserialize};
use analyzer::{WordStats};
use bson::Bson;
use url::Url;
use tokio;
use std::error::Error;
use r2d2::ManageConnection;


pub fn add_word(conn: &Database, word: String, stats: WordStats) -> Result<(), MongoEror> {
    let doc = doc! {
        "word" : word,
        "docs" : stats.docs.into_iter().map(Bson::from).collect::<Vec<_>>(),
        "positions" : stats.position.into_iter().map(Bson::from).collect::<Vec<_>>(),
        "word_length" : stats.word_length,
        "freq" : stats.freq
    };

    let coll = conn.collection("index");
    coll.insert_one(doc, None).map(drop)
}

pub fn establish_mongo_conn() -> r2d2::PooledConnection<r2d2_mongodb::MongodbConnectionManager>{
    let addr = Url::parse("mongodb://localhost:27017/admin").unwrap();
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(addr.host_str().unwrap_or("localhost"), addr.port().unwrap_or(27017))
            .with_db(&addr.path()[1..])
            .build()
    );

    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .unwrap();

    let conn = pool.get().unwrap();
    conn
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    Ok(())

}
