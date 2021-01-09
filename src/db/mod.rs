pub mod models;
pub mod repositories;
pub mod schema;

use derive_more::From;
use diesel::{
    backend::Backend, deserialize, r2d2::ConnectionManager, serialize, serialize::Output,
    types::FromSql, types::ToSql, AsExpression, SqliteConnection,
};
use r2d2::{Pool, PooledConnection};
use std::{io::Write, path::PathBuf};

#[derive(Debug, From)]
pub enum Error {
    NotFound,
    ConnectionError,
    OtherDbError(diesel::result::Error),
}

pub type DbResult<T> = Result<T, Error>;

#[derive(Clone)]
pub struct DbConnection {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

embed_migrations!("migrations");

impl DbConnection {
    pub async fn new(database_path: &PathBuf) -> Self {
        let db_url = &database_path.to_string_lossy().into_owned();
        DbConnection::new_from_url(db_url).await
    }

    pub async fn new_from_url(database_url: &str) -> Self {
        let conman = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder().max_size(15).build(conman).unwrap();
        embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout()).unwrap();
        DbConnection { pool }
    }
    // pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
    //     DbConnection { pool }
    // }

    pub fn get(&self) -> DbResult<PooledConnection<ConnectionManager<SqliteConnection>>> {
        let c: PooledConnection<ConnectionManager<SqliteConnection>> = self
            .pool
            .get_timeout(std::time::Duration::from_secs(12))
            .map_err(|e| Error::ConnectionError)?;
        Ok(c)
    }
}

// unsafe impl Send for DbConnection {}
// unsafe impl Sync for DbConnection {}

macro_rules! generate_uuid_field {
    ( $name:ident ) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
            AsExpression,
            FromSqlRow,
        )]
        #[sql_type = "diesel::sql_types::Text"]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new() -> $name {
                $name(uuid::Uuid::new_v4())
            }

            pub fn from_uuid(uuid: uuid::Uuid) -> $name {
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
                Ok($name::from_uuid(uuid_value))
            }
        }

        // TODO: UUID FromSql, ToSql for other types, like binary or very long integer?
        // 1. Add diesel feature = "uuid",
        // 2. Create FromSql and ToSql for  diesel::pg::types::sql_types::Uuid
        //
        // More about: https://docs.diesel.rs/diesel/pg/types/sql_types/struct.Uuid.html
    };
}

generate_uuid_field! {
    ArticleId
}

generate_uuid_field! {
    PageId
}

generate_uuid_field! {
    ImageId
}

generate_uuid_field! {
    UrlId
}
