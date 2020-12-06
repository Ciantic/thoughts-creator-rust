use diesel::SqliteConnection;

use super::models::Article;
use super::schema::articles::dsl::*;
use diesel::prelude::*;

impl Article {
    pub fn get_all(connection: &SqliteConnection) -> Vec<Article> {
        let foo = articles.limit(5).load::<Article>(connection);
        foo.expect("Error loading posts")
    }

    pub fn save(&self, connection: &SqliteConnection) {
        diesel::insert_into(articles)
            .values(self)
            .execute(connection);
        // insert_into(articles).values().execute(connection);

        diesel::update(articles)
            .set(self)
            .execute(connection)
            .unwrap();
    }
}
