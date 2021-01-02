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
    pub old_url: Option<String>,
    pub html: String,
}
pub async fn compile_markdown_file(path: &PathBuf) -> Result<CompiledMarkdown, Error> {
    let local_path = path.to_str().ok_or(Error::PathEncodingError)?.to_owned();
    let content = fs::read_to_string(&path).await.map_err(Error::IOError)?;
    let metadata = fs::metadata(&path).await.map_err(Error::IOError)?;
    let modified_on_disk: DateTime<Utc> = metadata.modified().map_err(Error::IOError)?.into();
    let (frontmatter, markdown) =
        frontmatter::get_frontmatter(&content).map_err(Error::FrontmatterParseError)?;

    let title = frontmatter.title.unwrap_or_else(|| first_title(&markdown));

    Ok(CompiledMarkdown {
        title,
        old_url: None,
        modified: chrono::Local::now().naive_utc(),
        modified_on_disk: modified_on_disk.naive_utc(),
        local_path,
        published: chrono::Local::now().naive_local(),
        html: markdown_to_html(&markdown).await,
    })
}

#[cfg(test)]
mod test_compile_markdown {
    use async_std::path::PathBuf;

    use super::{compile_markdown_file, Error};

    #[async_std::test]
    async fn test_compile() {
        let post = compile_markdown_file(&"./examples/articles/post01.md".into())
            .await
            .unwrap();
        assert_eq!(post.title, "First post")
    }

    #[async_std::test]
    async fn test_compile2() {
        let post = compile_markdown_file(&"./examples/articles/post02.md".into())
            .await
            .unwrap();
        assert_eq!(post.title, "Second post")
    }
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
    //     .trim_start_matches('#')
    //     .trim()
    //     .to_owned()
}

#[cfg(test)]
mod test_first_title {
    use super::first_title;

    #[test]
    fn test_first_title() {
        assert_eq!(first_title("\n this is not the title"), "");
        assert_eq!(first_title("sole line"), "sole line");
        assert_eq!(first_title("the title \n second line"), "the title");
        assert_eq!(first_title("# the title \r\n second line"), "the title");
        assert_eq!(first_title("## the title \r\n second line"), "the title");
    }
}
