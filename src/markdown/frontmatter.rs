use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::Deserialize;

#[derive(Debug, From)]
pub enum Error {
    // ParseError(serde::)
    YamlParseError(serde_yaml::Error),
}

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub old_url: Option<url::Url>,
}

pub fn get_frontmatter(markdown: &str) -> Result<(Frontmatter, String), Error> {
    if markdown.starts_with("---") {
        let parts: Vec<&str> = markdown.splitn(3, "---").collect();
        if parts.len() == 3 {
            let frontmatter_str = parts[1];
            let frontmatter = serde_yaml::from_str::<Frontmatter>(frontmatter_str)?;
            let markdown_str = parts[2];
            return Ok((
                frontmatter,
                // Remove new lines following `---`
                markdown_str
                    .trim_start_matches(|c| c == '\r' || c == '\n')
                    .into(),
            ));
        }
    }
    Ok((Frontmatter::default(), markdown.into()))
}

#[cfg(test)]
mod test_frontmatter {
    use super::{get_frontmatter, Frontmatter};
    use chrono::{TimeZone, Utc};
    use url::Url;

    #[test]
    fn test_frontmatter() {
        let (frontmatter, markdown) = get_frontmatter(
            &"---
            published: 2020-01-01 12:00:00 +03:00
            title: First post
            old_url: https://www.foo.com/path1/path2
            ---

            # The title

            Paragraph...
            "
            .replace("            ", ""),
        )
        .unwrap();

        let expected_frontmatter = Frontmatter {
            published: Utc.ymd(2020, 1, 1).and_hms(9, 0, 0).into(),
            old_url: Some(Url::parse("https://www.foo.com/path1/path2").unwrap()),
            title: Some("First post".into()),
        };

        assert_eq!(frontmatter, expected_frontmatter);

        assert_eq!(
            markdown,
            "# The title

            Paragraph...
            "
            .replace("            ", "")
        )
    }
}
