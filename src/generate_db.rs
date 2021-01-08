use crate::db::{ArticleId, PageId};
use crate::markdown::compile_markdown_file;
use crate::urls::convert_html_urls;
use crate::{db::models::Article, WatchMessage};
use crate::{db::DbConnection, FilesChange};
use async_std::channel::Sender;
use async_std::task::JoinHandle;
use derive_more::From;
use futures::future::join_all;
use glob::glob;
use std::path::PathBuf;

#[derive(Debug, From)]
pub enum Error {
    // IOError(std::io::Error),
    PatternError(glob::PatternError),
    DbError(crate::db::Error),
    CompileMarkdownError(crate::markdown::Error),
    UrlConvertError(crate::urls::Error),
    // UrlToFilePath,
}

async fn generate_article_db(
    article_file: &PathBuf,
    root_path: &PathBuf,
    pool: &DbConnection,
) -> Result<Vec<url::Url>, Error> {
    let markdown = compile_markdown_file(&article_file.into()).await?;
    let article_path = markdown.local_path.parent().unwrap();
    let converted =
        convert_html_urls(&markdown.html, &article_path.into(), &root_path.into()).await?;

    let article = Article {
        id: ArticleId::new(),
        html: converted.html,
        title: markdown.title,
        local_path: markdown.local_path.to_string_lossy().into_owned(),
        published: markdown.published.naive_utc(),
        modified: markdown.modified.naive_utc(),
        modified_on_disk: markdown.modified_on_disk.naive_utc(),

        // TODO: Colliding slugs?
        server_path: format!("/articles/{}", markdown.slug),
    };
    article.save(&pool).await?;

    Ok(converted.urls)
}

pub async fn generate_all(
    changes: Vec<FilesChange>,
    root_dir: &PathBuf,
    pool: &DbConnection,
    sender: &Sender<WatchMessage>,
) -> Result<(), Error> {
    let mut generate_threads: Vec<JoinHandle<()>> = vec![];

    for m in changes {
        match m {
            FilesChange::ArticlesChanged { files } => {
                for path in files {
                    let pool = pool.clone();
                    let root_dir = root_dir.clone();
                    let sender = sender.clone();
                    let thread = async_std::task::spawn(async move {
                        let urls = generate_article_db(&path, &root_dir, &pool).await;
                        match urls {
                            Ok(urls) => {
                                let _ = sender
                                    .send(WatchMessage::DbArticleCreated { path, urls })
                                    .await;
                            }
                            Err(error) => {
                                let _ = sender
                                    .send(WatchMessage::DbArticleError { path, error })
                                    .await;
                            }
                        };
                    });
                    generate_threads.push(thread);
                }
            }
            _ => (),
        }
    }

    // for path in page_files {
    //     let pool2 = pool.clone();
    //     let root_dir = root_dir.clone();
    //     let thread = async_std::task::spawn(async move {
    //         Ok(generate_article_db(&path, &root_dir, &pool2).await?)
    //     });
    //     generate_threads.push(thread);
    // }

    join_all(generate_threads).await;
    let _ = sender.send(WatchMessage::DbDone).await;
    Ok(())
}
