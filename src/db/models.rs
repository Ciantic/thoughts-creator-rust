use super::uuid::UUID;
use crate::db::schema::*;
use chrono::NaiveDateTime;
use chrono::Utc;

macro_rules! generate_uuid_field {
    ( $name:ident ) => {
        use diesel::{
            backend::Backend, deserialize, serialize, serialize::Output, types::FromSql,
            types::ToSql, AsExpression,
        };
        use std::io::Write;

        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            AsExpression,
            FromSqlRow,
        )]
        #[sql_type = "diesel::sql_types::Text"]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn generate() -> $name {
                $name(uuid::Uuid::new_v4())
            }

            pub fn new(uuid: uuid::Uuid) -> $name {
                $name(uuid)
            }
        }

        impl<DB> ToSql<diesel::sql_types::Text, DB> for $name
        where
            DB: Backend,
            String: ToSql<diesel::sql_types::Text, DB>,
        {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
                <String as ToSql<diesel::sql_types::Text, DB>>::to_sql(
                    &self.0.to_hyphenated().to_string(),
                    out,
                )
            }
        }

        impl<DB> FromSql<diesel::sql_types::Text, DB> for $name
        where
            DB: Backend,
            String: FromSql<diesel::sql_types::Text, DB>,
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                let db_value_str =
                    <String as FromSql<diesel::sql_types::Text, DB>>::from_sql(bytes)?;
                let uuid_value = uuid::Uuid::parse_str(&db_value_str)?;
                Ok($name::new(uuid_value))
            }
        }
    };
}

generate_uuid_field! {
    ArticleId
}

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
