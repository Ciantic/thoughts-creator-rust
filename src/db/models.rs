use super::ArticleId;
use crate::db::schema::*;
use chrono::NaiveDateTime;
use chrono::Utc;

#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub hash: ArticleId,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub modified_on_disk: NaiveDateTime,
    pub local_path: String,
    pub server_path: String,
    pub title: String,
    pub html: String,
}

impl Article {
    pub fn new() -> Article {
        Article {
            id: 0,
            hash: ArticleId::generate(),
            created: Utc::now().naive_utc(),
            modified: Utc::now().naive_utc(),
            modified_on_disk: Utc::now().naive_utc(),
            local_path: "".into(),
            server_path: "".into(),
            title: "".into(),
            html: "".into(),
        }
    }
}

#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Resource {
    pub id: i32,
    pub modified_on_disk: NaiveDateTime,
    pub local_path: String,
    pub server_path: String,
}

#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Image {
    pub id: i32,
    pub modified_on_disk: NaiveDateTime,
    pub width: i32,
    pub height: i32,
    pub local_path: String,
    pub server_path: String,
}

#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Url {
    pub id: i32,
    pub url: String,
}
