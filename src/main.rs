#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod db;
mod git;
mod markdown;
mod urls;

use crate::db::models::Article;
use async_std::task::JoinHandle;
use db::DbConnection;
use derive_more::From;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::embed_migrations;
use futures::future::join_all;
use glob::glob;
use markdown::compile_markdown_file;
use r2d2::Pool;
use std::{convert, path::PathBuf};
use urls::convert_html_urls;

embed_migrations!("migrations");
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct GenerateParams {
    pub article_dir: PathBuf,
    pub pages_dir: PathBuf,
    pub output_dir: PathBuf,
    pub root_dir: PathBuf,
    pub db_file: PathBuf,
    pub clean_output: bool,
}

#[derive(Debug, From)]
pub enum DbCreationError {
    // IOError(std::io::Error),
    DbError(db::Error),
    CompileMarkdownError(markdown::Error),
    UrlConvertError(urls::Error),
    UrlToFilePath,
}

async fn generate_article_db(
    article_file: &PathBuf,
    root_path: &PathBuf,
    pool: &DbConnection,
) -> Result<(Article, Vec<url::Url>), DbCreationError> {
    let markdown = compile_markdown_file(&article_file.into()).await?;
    let converted =
        convert_html_urls(&markdown.html, &article_file.into(), &root_path.into()).await?;

    let mut article = Article::new();
    article.title = markdown.title;
    article.local_path = markdown.local_path.to_string_lossy().into_owned();
    article.published = markdown.published.naive_utc();
    article.modified = markdown.modified.naive_utc();
    article.modified_on_disk = markdown.modified_on_disk.naive_utc();
    article.html = converted.html;
    article.server_path = format!("/articles/{}", markdown.slug); // TODO: Colliding slugs?
    article.save(&pool).await?;

    // get_relative_urls() -> url list
    //   relative_urls -> normalize url by article.server_path
    //   find article or page by the local_path
    //   rewrite the URL in HTML

    // get_external_urls(html) -> url list

    // generate_resources_db(&article).await?;
    // generate_images_db(&article).await?;

    Ok((article, converted.urls))
}

async fn generate_resources_db(article: &Article) -> Result<(), DbCreationError> {
    todo!()
}

async fn generate_images_db(article: &Article) -> Result<(), DbCreationError> {
    todo!()
}

async fn update_html_imagesizes(
    html: String,
    local_path: PathBuf,
    pool: &DbConnection,
) -> Result<String, DbCreationError> {
    todo!()
}

async fn layout_article(article: Article, pool: &DbConnection) -> Result<String, DbCreationError> {
    todo!()
}

async fn generate_article(
    params: &GenerateParams,
    article_file: &PathBuf,
    pool: &DbConnection,
) -> Result<(), DbCreationError> {
    let (article, article_urls) =
        generate_article_db(&article_file, &params.root_dir, pool).await?;

    for url in article_urls {
        if url.scheme() == "file" {
            let path = url
                .to_file_path()
                .map_err(|_| DbCreationError::UrlToFilePath)?;
            match path.extension() {
                Some(ext) => match &ext.to_string_lossy() as &str {
                    "png" | "jpg" | "gif" | "svg" => {
                        todo!()
                    }
                    "md" | "markdown" => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                },
                None => {
                    todo!()
                }
            }
        } else {
        }
    }

    Ok(())
}

#[derive(From, Debug)]
enum GenerateError {
    PatternError(glob::PatternError),
}

enum GenerateInput {
    Article(PathBuf),
    Page(PathBuf),
}

async fn generate(params: &GenerateParams, pool: DbConnection) -> Result<(), GenerateError> {
    // Initialize the DB
    let _ = embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout());

    // Get input markdown files
    let article_files = glob(&format!("{}/**/*.md", params.article_dir.to_string_lossy()))?
        .filter_map(Result::ok)
        .map(GenerateInput::Article);
    let page_files = glob(&format!("{}/**/*.md", params.pages_dir.to_string_lossy()))?
        .filter_map(Result::ok)
        .map(GenerateInput::Page);
    let files = article_files.chain(page_files);

    let mut generate_threads: Vec<JoinHandle<Result<(), DbCreationError>>> = vec![];
    for entry in files {
        let pool2 = pool.clone();
        match entry {
            GenerateInput::Article(path) => {
                let params = params.clone();
                let thread = async_std::task::spawn(async move {
                    generate_article(&params, &path, &pool2).await
                });
                generate_threads.push(thread);
            }

            GenerateInput::Page(e) => println!("{:?}", e),
        }
    } // db!()

    let foo = join_all(generate_threads);

    let res = foo.await;
    Ok(())
    // let futures = generate_threads.iter().fold(|n| n.join());
}

#[derive(Debug)]
enum MainError {
    IOError,
}

enum WatchMessage {
    Started,
    RootChanged { root_files: Vec<PathBuf> },
    ResourcesChanged { resource_files: Vec<PathBuf> },
    ArticlesChanged { article_files: Vec<PathBuf> },
    PagesChanged { page_files: Vec<PathBuf> },
}

enum GenerationMessage {
    StartDbReconciliation,
    EndDbReconciled,
    Layouting,
    DoneLayouting,
}

#[async_std::main]
async fn main() -> Result<(), MainError> {
    let conman = ConnectionManager::<SqliteConnection>::new(".cache.db");
    let pool = Pool::builder().max_size(15).build(conman).unwrap();

    let _ = generate(
        &GenerateParams {
            article_dir: ".\\examples\\articles".into(),
            pages_dir: ".\\examples\\pages".into(),
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
