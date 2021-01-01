use chrono::Duration;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::Pool;

use super::{models::Article, DbConnection};
use super::{schema::articles::dsl::*, DbResult};
use diesel::prelude::*;

impl Article {
    pub async fn get_all(dbc: &DbConnection) -> DbResult<Vec<Article>> {
        let foo = articles.limit(5).load::<Article>(&dbc.get()?);
        Ok(foo.expect("Error loading posts"))
    }

    pub async fn save(&self, dbc: &DbConnection) -> DbResult<()> {
        // SQLite and MySQL
        diesel::replace_into(articles)
            .values(self)
            .execute(&dbc.get()?)
            .unwrap();

        Ok(())
        // PG (and upcoming 1.4.6 Diesel release):
        // diesel::insert_into(articles)
        //     .values(self)
        //     .on_conflict(id)
        //     .do_update()
        //     .set(self)
        //     .execute(connection);
    }

    pub async fn delete(&self, dbc: &DbConnection) -> DbResult<()> {
        diesel::delete(articles)
            .filter(id.eq(&self.id))
            .execute(&dbc.get()?)
            .unwrap();

        Ok(())
    }
}
