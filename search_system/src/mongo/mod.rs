use bson::{bson, doc};
use chrono::offset::Utc;
use mongodb::Error as MongoEror;
use r2d2::Pool;
use mongodb::db::{Database, ThreadedDatabase};
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use serde::{Deserialize, Serialize};
use analyzer::{WordStats, create_tokens_list, create_index};
use analyzer::{read_files_from_dir, read_text};
use bson::Bson;
use url::Url;
use tracing::{info, instrument};

#[derive(Serialize, Deserialize, Debug)]
pub struct WordOut {
    pub word: String,
    pub docs: Vec<i32>,
    pub positions: Vec<Vec<i32>>,
    pub word_length: i32,
    pub freq: i32
}

pub type MongoConn = r2d2::PooledConnection<r2d2_mongodb::MongodbConnectionManager>;
pub type MongoPool = r2d2::Pool<r2d2_mongodb::MongodbConnectionManager>;

pub struct Mongo;


impl Mongo {
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
        let check = Self::check_ifexists(conn, &word).await;
        if check {
            let filter = doc!{ "word": word };
            let update = doc!{ "$addToSet": { 
                "docs" : { "$each" : stats.docs.into_iter().map(Bson::from).collect::<Vec<_>>()},
                },
                "$push" : {
                    "positions" : { "$each" : stats.position.into_iter().map(Bson::from).collect::<Vec<_>>() }
                },
                "$inc" : {"freq" : stats.freq}
            };
            println!("added a document");
            conn.collection("index").update_one(filter, update, None).map(drop)
        } else {
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

    pub async fn add_many_words(conn: &Database, word: String, stats: WordStats) -> Result<(), MongoEror> {
        let check = Self::check_ifexists(conn, &word).await;
        if check {
            let filter = doc!{ "word": word };
            let update = doc!{ "$addToSet": { 
                "docs" : { "$each" : stats.docs.into_iter().map(Bson::from).collect::<Vec<_>>()},
                },
                "$push" : {
                    "positions" : { "$each" : stats.position.into_iter().map(Bson::from).collect::<Vec<_>>() }
                },
                "$inc" : {"freq" : stats.freq}
            };
            println!("added a document");
            conn.collection("index").update_one(filter, update, None).map(drop)
        } else {
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

    #[instrument]
    pub async fn establish_mongo_conn() -> MongoPool {
        let addr = Url::parse("mongodb://localhost:27017/admin").unwrap();
        info!("Establishing MongoDB connection");
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

        pool
    }

}
