use crate::db::schema::{users};

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid, 
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
