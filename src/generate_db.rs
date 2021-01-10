use crate::{db::models::Article, Message};
use crate::{db::DbConnection, FilesChange};
use crate::{
    db::{ArticleId, PageId},
    GenerateParams,
};
use crate::{markdown::compile_markdown_file, utils::normalize};
use crate::{urls::convert_html_urls, utils::normalize_sync};
use async_std::channel::Sender;
use async_std::path::PathBuf;
use async_std::task::JoinHandle;
use derive_more::From;
use futures::future::join_all;
use glob::glob;

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

async fn generate_all(
    changes: Vec<FilesChange>,
    root_dir: &PathBuf,
    pool: &DbConnection,
    sender: &Sender<Message>,
) {
    let mut generate_tasks: Vec<JoinHandle<()>> = vec![];

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
                                let _ = sender.send(Message::DbArticleCreated { path, urls }).await;
                            }
                            Err(error) => {
                                let _ = sender.send(Message::DbArticleError { path, error }).await;
                            }
                        };
                    });
                    generate_tasks.push(thread);
                }
            }
            FilesChange::PagesChanged { files } => {
                //
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

    join_all(generate_tasks).await;
    let _ = sender.send(Message::DbGenerated).await;
}

pub async fn sync(
    params: &GenerateParams,
    dbc: &DbConnection,
    sender: &Sender<Message>,
) -> Result<JoinHandle<()>, Error> {
    let article_dir = params.article_dir.clone();
    let pages_dir = params.pages_dir.clone();
    let root_dir = params.root_dir.clone();
    // Get input markdown files
    let article_files = glob(&format!("{}/**/*.md", article_dir.to_string_lossy()))?
        .filter_map(Result::ok)
        .map(|f| normalize_sync(&f).unwrap().into())
        .collect::<Vec<PathBuf>>();

    let page_files = glob(&format!("{}/**/*.md", pages_dir.to_string_lossy()))?
        .filter_map(Result::ok)
        .map(|f| normalize_sync(&f).unwrap().into())
        .collect::<Vec<PathBuf>>();

    Article::clean_non_existing(&dbc, article_files.as_slice()).await?;

    // Initially, we assume all files changed, before watch starts
    let msgs = vec![
        FilesChange::ArticlesChanged {
            files: article_files,
        },
        FilesChange::PagesChanged { files: page_files },
    ];

    let dbc = dbc.clone();
    let sender = sender.clone();
    Ok(async_std::task::spawn(async move {
        generate_all(msgs, &root_dir, &dbc, &sender).await
    }))
}
