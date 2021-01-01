use async_std::{fs::read_to_string, path::PathBuf};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::expression::NonAggregate;

use super::{frontmatter, to_html::markdown_to_html};

#[derive(Debug)]
pub enum Error {
    Na,
    FrontmatterParseError(frontmatter::Error),
    IOError(std::io::Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct CompiledMarkdown {
    pub title: String,
    pub published: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub old_url: Option<String>,
    pub html: String,
}

/// Get markdown title
fn first_title(str: &str) -> String {
    // Get first line, and remove leading hashes
    str.lines()
        .next()
        .unwrap_or(str)
        .trim_start_matches('#')
        .trim()
        .to_owned()

    // TODO: Does iterator approach differ from this?
    // str.chars()
    //     .take_while(|&c| c != '\n' || c != '\r')
    //     .collect::<String>()
    //     .trim()
    //     .to_owned()
}

pub async fn compile_markdown_file(path: PathBuf) -> Result<CompiledMarkdown, Error> {
    let content = read_to_string(path).await.map_err(Error::IOError)?;
    let (frontmatter, markdown) =
        frontmatter::get_frontmatter(&content).map_err(Error::FrontmatterParseError)?;

    let title = match frontmatter.title {
        Some(s) => s,
        None => first_title(&markdown),
    };

    Ok(CompiledMarkdown {
        title,
        html: markdown_to_html(&markdown).await,
        old_url: None,
        modified: chrono::Local::now().naive_local(),
        published: chrono::Local::now().naive_local(),
    })
}
