use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    // ParseError(serde::)
    ParseError(serde_yaml::Error),
}

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub published: Option<NaiveDateTime>,
    pub old_url: Option<String>,
}

pub fn get_frontmatter(markdown: &str) -> Result<(Frontmatter, String), Error> {
    if markdown.starts_with("---") {
        let parts: Vec<&str> = markdown.splitn(3, "---").collect();
        if parts.len() == 3 {
            let frontmatter_str = parts[1];
            let frontmatter =
                serde_yaml::from_str::<Frontmatter>(frontmatter_str).map_err(Error::ParseError)?;
            let markdown_str = parts[2];
            return Ok((frontmatter, markdown_str.into()));
        }
    }
    Ok((Frontmatter::default(), markdown.into()))
}
