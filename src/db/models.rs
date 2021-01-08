use super::{ArticleId, ImageId, PageId, UrlId};
use crate::db::schema::*;
use chrono::NaiveDateTime;
use chrono::Utc;

#[derive(
    Debug, Queryable, Identifiable, Insertable, AsChangeset, serde::Serialize, serde::Deserialize,
)]
pub struct Article {
    pub id: ArticleId,
    pub published: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub modified_on_disk: NaiveDateTime,
    pub local_path: String,
    pub server_path: String,
    pub title: String,
    pub html: String,
}

// impl Article {
//     pub fn new() -> Article {
//         Article {
//             id: ArticleId::generate(),
//             hash: "".into(),
//             published: Utc::now().naive_utc(),
//             modified: Utc::now().naive_utc(),
//             modified_on_disk: Utc::now().naive_utc(),
//             local_path: "".into(),
//             server_path: "".into(),
//             title: "".into(),
//             html: "".into(),
//         }
//     }
// }

// #[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
// pub struct Resource {
//     pub id: ResourceId,
//     pub modified_on_disk: NaiveDateTime,
//     pub local_path: String,
//     pub server_path: String,
// }

#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Image {
    pub id: ImageId,
    pub modified_on_disk: NaiveDateTime,
    pub width: i32,
    pub height: i32,
    pub local_path: String,
    pub server_path: String,
}

#[derive(Debug, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Url {
    pub id: UrlId,
    pub url: String,
}
