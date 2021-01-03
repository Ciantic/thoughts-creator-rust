#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod db;
mod git;
mod markdown;

use crate::db::models::Article;
use async_std::channel::unbounded;
use async_std::path::PathBuf;
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

embed_migrations!("migrations");
#[derive(serde::Serialize, serde::Deserialize)]
struct GenerateParams {
    pub article_dir: String,
    pub output_dir: String,
    pub root_dir: String,
    pub db_file: String,
    pub clean_output: bool,
}

#[derive(Debug, From)]
enum Error {
    IOError(std::io::Error),
    DbError(db::Error),
    CompileMarkdownError(markdown::Error),
}

async fn generate_article_db(file: PathBuf, pool: &DbConnection) -> Result<Article, Error> {
    let cm = compile_markdown_file(&file).await?;
    let mut article = Article::new();
    article.title = cm.title;
    article.local_path = cm.local_path.to_string_lossy().into_owned();
    article.created = cm.published.naive_utc();
    article.modified = cm.modified.naive_utc();
    article.modified_on_disk = cm.modified_on_disk.naive_utc();
    article.html = cm.html;

    // TODO: Colliding slugs?
    article.server_path = format!("/articles/{}", cm.slug);
    article.save(&pool).await?;

    generate_resources_db(article);
    generate_images_db(article);

    Ok(article)
}

async fn generate_resources_db(article: Article) -> Result<(), Error> {
    todo!()
}

async fn generate_images_db(article: Article) -> Result<(), Error> {
    todo!()
}

async fn update_html_imagesizes(
    html: String,
    local_path: PathBuf,
    pool: &DbConnection,
) -> Result<String, Error> {
    todo!()
}

async fn layout_article(article: Article, pool: &DbConnection) -> Result<String, Error> {
    todo!()
}

async fn generate_article(file: PathBuf, pool: &DbConnection) -> Result<(), Error> {
    let article = generate_article_db(file, pool).await?;
    Ok(())
}

async fn generate(params: &GenerateParams, pool: DbConnection) {
    let files_input = format!("{}/**/*.md", params.article_dir);
    let files = glob(&files_input);

    // Initialize the DB
    let _ = embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout());
    let mut generate_threads: Vec<JoinHandle<Result<(), Error>>> = vec![];

    for entry in files.unwrap() {
        let pool2 = pool.clone();

        match entry {
            Ok(path) => {
                let thread = async_std::task::spawn(async move {
                    println!("path {}", path.display());
                    generate_article_db(path.into(), &pool2).await?;
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
