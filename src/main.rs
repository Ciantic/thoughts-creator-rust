#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod db;
mod generate_db;
mod git;
mod markdown;
mod urls;

use crate::db::models::Article;
use async_std::{
    channel::{unbounded, Sender},
    task::JoinHandle,
};
use db::DbConnection;
use derive_more::From;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::embed_migrations;
use futures::future::join_all;
use generate_db::generate_all;
use glob::glob;
use markdown::compile_markdown_file;
use r2d2::Pool;
use std::{convert, path::PathBuf, time::Duration};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct GenerateParams {
    pub article_dir: PathBuf,
    pub pages_dir: PathBuf,
    pub output_dir: PathBuf,
    pub root_dir: PathBuf,
    pub db_file: PathBuf,
    pub clean_output: bool,
}

// async fn generate_resources_db(article: &Article) -> Result<(), DbCreationError> {
//     todo!()
// }

// async fn generate_images_db(article: &Article) -> Result<(), DbCreationError> {
//     todo!()
// }

// async fn update_html_imagesizes(
//     html: String,
//     local_path: PathBuf,
//     pool: &DbConnection,
// ) -> Result<String, DbCreationError> {
//     // for url in article_urls {
//     //     if url.scheme() == "file" {
//     //         let path = url
//     //             .to_file_path()
//     //             .map_err(|_| DbCreationError::UrlToFilePath)?;
//     //         match path.extension() {
//     //             Some(ext) => match &ext.to_string_lossy() as &str {
//     //                 "png" | "jpg" | "gif" | "svg" => {
//     //                     todo!()
//     //                 }
//     //                 "md" | "markdown" => {
//     //                     todo!()
//     //                 }
//     //                 _ => {
//     //                     todo!()
//     //                 }
//     //             },
//     //             None => {
//     //                 todo!()
//     //             }
//     //         }
//     //     } else {
//     //     }
//     // }
//     todo!()
// }

// async fn layout_article(article: Article, pool: &DbConnection) -> Result<String, DbCreationError> {
//     todo!()
// }

// async fn generate_article(
//     params: &GenerateParams,
//     article_file: &PathBuf,
//     pool: &DbConnection,
// ) -> Result<(), DbCreationError> {
//     let (article, article_urls) =
//         generate_article_db(&article_file, &params.root_dir, pool).await?;

//     Ok(())
// }

#[derive(From, Debug)]
enum GenerateError {
    PatternError(glob::PatternError),
}

enum GenerateInput {
    Article(PathBuf),
    Page(PathBuf),
}

async fn generate(params: &GenerateParams) -> Result<(), GenerateError> {
    let _ = async_std::fs::remove_file(&params.db_file).await;
    let pool = DbConnection::new(&params.db_file).await;

    // Initialize the DB
    let (sender, receiver) = unbounded();
    let article_dir = params.article_dir.clone();
    let pages_dir = params.pages_dir.clone();
    let root_dir = params.root_dir.clone();

    // Get input markdown files
    let article_files =
        glob(&format!("{}/**/*.md", article_dir.to_string_lossy()))?.filter_map(Result::ok);

    let page_files =
        glob(&format!("{}/**/*.md", pages_dir.to_string_lossy()))?.filter_map(Result::ok);

    // Initially, we assume all files changed, before watch starts
    let msgs = vec![
        FilesChange::ArticlesChanged {
            files: article_files.collect(),
        },
        FilesChange::PagesChanged {
            files: page_files.collect(),
        },
    ];

    async_std::task::spawn(async move { generate_all(msgs, &root_dir, &pool, &sender).await });

    loop {
        match receiver.recv().await {
            Ok(msg) => match msg {
                WatchMessage::DbDone => {
                    //
                    println!("Done!");
                    break;
                }
                m => {
                    println!("{:?}", m);
                    //
                }
            },
            Err(er) => {
                println!("EMPTY! {}", er);
                break;
            }
        }
    }

    //

    Ok(())
}

#[derive(Debug)]
enum MainError {
    IOError,
}

#[derive(Debug)]
pub enum FilesChange {
    RootChanged { root_files: Vec<PathBuf> },
    ArticlesChanged { files: Vec<PathBuf> },
    PagesChanged { files: Vec<PathBuf> },
    ArticlesDeleted { files: Vec<PathBuf> },
    PagesDeleted { files: Vec<PathBuf> },
}

#[derive(Debug)]
pub enum WatchMessage {
    Changes(Vec<FilesChange>),
    DbArticleError {
        path: PathBuf,
        error: generate_db::Error,
    },
    DbArticleCreated {
        path: PathBuf,
        urls: Vec<url::Url>,
    },
    DbDone,
}

#[async_std::main]
async fn main() -> Result<(), MainError> {
    let _ = generate(&GenerateParams {
        article_dir: ".\\examples\\articles".into(),
        pages_dir: ".\\examples\\pages".into(),
        clean_output: true,
        db_file: ".cache.db".into(),
        output_dir: ".\\.out".into(),
        root_dir: ".".into(),
    })
    .await;
    Ok(())
}
