use bson::{bson, doc};
use chrono::offset::Utc;
use mongodb::Error as MongoEror;
use r2d2::Pool;
use mongodb::db::{Database, ThreadedDatabase};
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use serde_derive::{Deserialize, Serialize};
use analyzer::{WordStats};
use bson::Bson;
use url::Url;
use tokio;
use std::error::Error;
use std::collections::HashSet;
use r2d2::ManageConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct WordOut {
    pub word: String,
    pub docs: Vec<i32>,
    pub positions: Vec<Vec<i32>>,
    pub word_length: i32,
    pub freq: i32
}

pub async fn check_ifexists(conn: &Database, word: &String) -> bool{
    let word = conn.collection("index").find_one(
        Some(doc! {
              "word": format!("{}", word)
        }), None).unwrap();
    let exists: bool;
    if let Some(word) = word {
        exists = true;
    } else {
        exists = false;
    }
    exists
}

// Need to fix the conversion from bson here, need to happen before db insertion
pub async fn add_word(conn: &Database, word: String, stats: WordStats) -> Result<(), MongoEror> {
    let check = check_ifexists(conn, &word).await;
    if check {
        println!("got here");
        let filter = doc!{ "word": word };
        let update = doc!{ "$addToSet": { 
            "docs" : { "$each" : stats.docs.into_iter().map(Bson::from).collect::<Vec<_>>()},
            },
            "$push" : {
                "positions" : stats.position.into_iter().map(Bson::from).collect::<Vec<_>>()
            },
            "$inc" : {"freq" : stats.freq}
        };
        conn.collection("index").update_one(filter, update, None).map(drop)
    } else {
        println!("got here2");
        let doc = doc! {
            "word" : word,
            "docs" : stats.docs.into_iter().map(Bson::from).collect::<Vec<_>>(),
            "positions" : [stats.position.into_iter().map(Bson::from).collect::<Vec<_>>()],
            "word_length" : stats.word_length,
            "freq" : stats.freq
        };
    
        let coll = conn.collection("index");
        coll.insert_one(doc, None).map(drop)
    }
}

pub async fn list_index(conn: &Database) -> Result<Vec<WordOut>, MongoEror> {
    conn.collection("index").find(None, None).unwrap()
        .try_fold(Vec::new(), | mut vec, doc| {
            let doc = doc.unwrap();
            let wordout: WordOut = bson::from_bson(Bson::Document(doc)).unwrap();
            vec.push(wordout);
            Ok(vec)
        })
}

pub async fn establish_mongo_conn() -> r2d2::PooledConnection<r2d2_mongodb::MongodbConnectionManager>{
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
    let pool = establish_mongo_conn().await;
    let mut hash = HashSet::new();
    hash.insert(3);
    hash.insert(4);
    
    let wordstat = WordStats {
        docs: hash,
        position: vec![1, 45, 45],
        word_length: 5,
        freq: 1
    };

    add_word(&pool, "shit".to_owned(), wordstat).await;
    let word_out = list_index(&pool).await.unwrap();
    println!("{:?}", word_out);
    Ok(())

}
