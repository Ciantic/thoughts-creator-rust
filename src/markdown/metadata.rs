use async_std::{fs, path::PathBuf};
use chrono::{DateTime, NaiveDateTime, Utc};

use super::{frontmatter, to_html::markdown_to_html};

#[derive(Debug)]
pub enum Error {
    PathEncodingError,
    FrontmatterParseError(frontmatter::Error),
    IOError(std::io::Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct CompiledMarkdown {
    pub title: String,
    pub published: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub modified_on_disk: NaiveDateTime,
    pub local_path: String,
    pub old_url: Option<url::Url>,
    pub html: String,
}

pub async fn compile_markdown_file(path: &PathBuf) -> Result<CompiledMarkdown, Error> {
    let local_path = path.to_str().ok_or(Error::PathEncodingError)?.to_owned();
    let content = fs::read_to_string(&path).await.map_err(Error::IOError)?;
    let metadata = fs::metadata(&path).await.map_err(Error::IOError)?;
    let modified_on_disk: DateTime<Utc> = metadata.modified().map_err(Error::IOError)?.into();
    let (frontmatter, markdown_all) =
        frontmatter::get_frontmatter(&content).map_err(Error::FrontmatterParseError)?;
    let (title, markdown): (String, String) = match frontmatter.title {
        None => {
            // Since front matter does not contain title, this takes the first
            // line in markdown as title (removing any hashes)

            // Notice: It doesn't work with underlined header format at the moment
            let (title, rest) = first_title_rest(&markdown_all);
            (title.to_owned(), rest.to_owned())
        }
        Some(title) => (title, markdown_all.to_owned()),
    };
    let html = markdown_to_html(&markdown).await;

    Ok(CompiledMarkdown {
        title,
        old_url: frontmatter.old_url,
        modified: chrono::Local::now().naive_utc(),
        modified_on_disk: modified_on_disk.naive_utc(),
        local_path,
        published: chrono::Local::now().naive_local(),
        html,
    })
}

#[cfg(test)]
mod test_compile_markdown {
    use super::compile_markdown_file;

    #[async_std::test]
    async fn test_compile_post01() {
        let post = compile_markdown_file(&"./examples/articles/post01.md".into())
            .await
            .unwrap();
        assert_eq!(post.title, "First post");
    }

    #[async_std::test]
    async fn test_compile_post02() {
        let post = compile_markdown_file(&"./examples/articles/post02.md".into())
            .await
            .unwrap();
        // Notice that the title was separated from the rest of the markdown
        assert_eq!(post.title, "Second post");
        assert!(post.html.starts_with("<p>Lorem ipsum dolor sit amet!"));
    }
}

/// Get first title, and rest of the markdown
fn first_title_rest(str: &str) -> (&str, &str) {
    let mut parts = str.splitn(2, '\n');
    (
        parts.next().unwrap().trim_start_matches('#').trim(),
        parts
            .next()
            .unwrap_or("")
            .trim_start_matches(|c| c == '\r' || c == '\n'),
    )
}

#[cfg(test)]
mod test_first_title {
    use super::first_title_rest;

    #[test]
    fn test_first_title_rest() {
        assert_eq!(first_title_rest(""), ("", ""));
        assert_eq!(
            first_title_rest("\n this is not the title"),
            ("", " this is not the title")
        );
        assert_eq!(first_title_rest("sole line"), ("sole line", ""));

        assert_eq!(
            first_title_rest("the title \n second line"),
            ("the title", " second line")
        );

        // Trim the start of second line from new lines:
        assert_eq!(
            first_title_rest("the title \n\n\r\n second line"),
            ("the title", " second line")
        );

        // But it doesn't affect the third line
        assert_eq!(
            first_title_rest("## the title \r\n second line \r\n third line"),
            ("the title", " second line \r\n third line")
        );
    }
}
