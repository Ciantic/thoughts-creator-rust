use async_std::path::PathBuf;
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

    pub async fn clean_non_existing(
        dbc: &DbConnection,
        existing_article_files: &[PathBuf],
    ) -> DbResult<usize> {
        let local_paths = existing_article_files
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>();

        Ok(diesel::delete(articles)
            .filter(local_path.ne_all(local_paths))
            .execute(&dbc.get()?)?)
    }
}

#[cfg(test)]
mod test {
    use crate::db::DbConnection;

    use super::super::ArticleId;
    use super::Article;

    async fn create_test_articles(dbc: &DbConnection) {
        let test1 = Article {
            html: "".into(),
            id: ArticleId::new(),
            local_path: "./examples/post01.md".into(),
            modified: chrono::Local::now().naive_utc(),
            modified_on_disk: chrono::Local::now().naive_utc(),
            published: chrono::Local::now().naive_utc(),
            server_path: "/examples/post01/".into(),
            title: "Example post 01".into(),
        };
        let test2 = Article {
            html: "".into(),
            id: ArticleId::new(),
            local_path: "./examples/post02.md".into(),
            modified: chrono::Local::now().naive_utc(),
            modified_on_disk: chrono::Local::now().naive_utc(),
            published: chrono::Local::now().naive_utc(),
            server_path: "/examples/post02/".into(),
            title: "Example post 02".into(),
        };
        let test3 = Article {
            html: "".into(),
            id: ArticleId::new(),
            local_path: "./examples/non-existing.md".into(),
            modified: chrono::Local::now().naive_utc(),
            modified_on_disk: chrono::Local::now().naive_utc(),
            published: chrono::Local::now().naive_utc(),
            server_path: "/examples/non-existing/".into(),
            title: "Example non existing".into(),
        };

        let _ = test1.save(&dbc).await;
        let _ = test2.save(&dbc).await;
        let _ = test3.save(&dbc).await;
    }

    #[async_std::test]
    async fn test_clean_non_existing() {
        let dbc = DbConnection::new_from_url(":memory:").await.unwrap();
        create_test_articles(&dbc).await;

        assert_eq!(Article::get_all(&dbc).await.unwrap().len(), 3);

        Article::clean_non_existing(
            &dbc,
            &["./examples/post01.md".into(), "./examples/post02.md".into()],
        )
        .await
        .unwrap();

        assert_eq!(Article::get_all(&dbc).await.unwrap().len(), 2);
    }
}
