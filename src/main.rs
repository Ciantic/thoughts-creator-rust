#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod db;
mod generate_db;
mod git;
mod markdown;
mod urls;
mod utils;

use crate::db::models::Article;
use async_std::path::PathBuf;
use async_std::{channel::unbounded, task::JoinHandle};
use db::DbConnection;
use derive_more::From;
use generate_db::sync;
use utils::normalize;

#[derive(Clone)]
pub struct GenerateParams {
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
    DbError(crate::db::Error),
    PatternError(glob::PatternError),
}

async fn generate(params: &GenerateParams) -> Result<(), GenerateError> {
    // Re-create database on each run
    // let _ = async_std::fs::remove_file(&params.db_file).await;
    let pool = DbConnection::new(&params.db_file.clone().into()).await?;

    // Initialize the DB
    let (sender, receiver) = unbounded();

    let mut generate_db_task: Option<JoinHandle<()>> = None;

    // Initially, run Sync
    let _ = sender.send(Message::Sync).await;

    loop {
        match receiver.recv().await {
            Ok(msg) => match msg {
                Message::Sync => {
                    if let Some(thread) = generate_db_task {
                        thread.cancel().await;
                    }
                    generate_db_task = Some(sync(&params, &pool, &sender).await.unwrap());
                }
                Message::DbGenerated => {
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
pub enum Message {
    Sync,
    Changes(Vec<FilesChange>),
    DbArticleError {
        path: PathBuf,
        error: generate_db::Error,
    },
    DbArticleCreated {
        path: PathBuf,
        urls: Vec<url::Url>,
    },
    DbGenerated,
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
