#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod db;
mod markdown;

use async_std::path::PathBuf;

use crate::db::models::Article;
use db::{DbConnection, DbError};
use diesel::{
    prelude::*, r2d2::ConnectionManager, serialize::Output, sql_types::Timestamp, sqlite::Sqlite,
    types::ToSql,
};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::embed_migrations;
use futures::future::join_all;
use glob::glob;

use async_std::task::JoinHandle;
use markdown::compile_markdown_file;
use r2d2::Pool;
use yew::prelude::*;

embed_migrations!("migrations");
#[derive(serde::Serialize, serde::Deserialize)]
struct GenerateParams {
    pub article_dir: String,
    pub output_dir: String,
    pub root_dir: String,
    pub db_file: String,
    pub clean_output: bool,
}

#[derive(Debug)]
enum GenerateError {
    IOError,
    FileReadError(std::io::Error),
    DbConnectionError(DbError),
    CompileMarkdownError(markdown::Error),
}

async fn generate_article(file: PathBuf, pool: &DbConnection) -> Result<(), GenerateError> {
    // let dbcon = pool.get().map_err(GenerateError::DbConnectionError)?;
    let markdown = async_std::fs::read_to_string(&file)
        .await
        .map_err(GenerateError::FileReadError)?;

    let cm = compile_markdown_file(&file)
        .await
        .map_err(GenerateError::CompileMarkdownError)?;

    let mut article = Article::new();
    article.title = "foo".into();
    article.local_path = file.to_string_lossy().into();
    article.html = cm.html;
    let _ = article.save(&pool).await;
    Ok(())
}

async fn generate(params: &GenerateParams, pool: DbConnection) {
    let files_input = format!("{}/**/*.md", params.article_dir);
    let files = glob(&files_input);

    // Initialize the DB
    let _ = embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout());
    let mut generate_threads: Vec<JoinHandle<Result<(), GenerateError>>> = vec![];

    for entry in files.unwrap() {
        let pool2 = pool.clone();

        match entry {
            Ok(path) => {
                let thread = async_std::task::spawn(async move {
                    println!("path {}", path.display());
                    generate_article(path.into(), &pool2).await?;
                    Ok(())
                });
                generate_threads.push(thread);
            }

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    } // db!()

    let foo = join_all(generate_threads);

    let res = foo.await;
    // let futures = generate_threads.iter().fold(|n| n.join());
}

#[derive(Debug)]
enum MainError {
    IOError,
}

#[async_std::main]
async fn main() -> Result<(), MainError> {
    let conman = ConnectionManager::<SqliteConnection>::new(".cache.db");
    let pool = Pool::builder().max_size(15).build(conman).unwrap();

    generate(
        &GenerateParams {
            article_dir: ".\\examples".into(),
            clean_output: true,
            db_file: ":memory:".into(),
            output_dir: ".\\.out".into(),
            root_dir: ".".into(),
        },
        DbConnection::new(pool),
    )
    .await;
    // let conn = init_db();

    // let thread = std::thread::spawn(move || {
    //     println!("OPEN THREAD");
    //     let conn = pool.get().unwrap();
    //     let _ = embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
    //     Article::new().save(&conn);
    //     Article::new().save(&conn);
    //     let results2 = Article::get_all(&conn);
    //     println!("{:?}", results2);
    // });

    // thread.join().unwrap();

    // std::thread::

    // let content = html! {
    //     <html>
    //     <div class="thing">
    //     </div>
    //     </html>
    // };
    // println!("html {:?}", content);
    Ok(())
}
