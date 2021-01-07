use async_std::path::PathBuf;
use derive_more::From;
use regex::{Captures, Regex};
use url::Url;

pub struct RelativeUrl(String);

impl RelativeUrl {
    pub fn normalize(current_path: AbsoluteUrl) -> AbsoluteUrl {
        let AbsoluteUrl(path) = current_path;
        todo!()
    }
}

pub struct AbsoluteUrl(String);

pub fn get_external_urls(html: String) -> Vec<Url> {
    todo!()
}

pub fn get_relative_urls(html: String) -> Vec<RelativeUrl> {
    todo!()
}

#[derive(Debug, From)]
pub enum Error {
    CanonicalizationFailed(std::io::Error),
    UrlCreationFailed,
    UrlParsingFailed(url::ParseError),
}

#[derive(Debug)]
pub struct NormalizedHtml {
    html: String,
    urls: Vec<Url>,
}

/// Normalizes all relative urls to be absolute file:// urls
pub async fn normalize_html_relative_urls(
    html: &str,
    current_path: &PathBuf,
    root_path: &PathBuf,
) -> Result<NormalizedHtml, Error> {
    let mut res = NormalizedHtml {
        html: "".into(),
        urls: vec![],
    };

    // Match all urls
    let reg = Regex::new(r#" (href|src)="([^"]*?)""#).unwrap();

    // let mut end = 0;
    let mut last_pos = html.len();

    // Notice: Regex::replace_all with FnMut does not work here, I need this to
    // be async and be able to return Error if needed.

    // Capture positions must be iterated in reverse order for replacing
    let captures_list = reg.captures_iter(html).collect::<Vec<_>>();
    for capture in captures_list.iter().rev() {
        // All matches is guaranteed to have second group
        let rmatch = capture.get(2).unwrap();

        let start = rmatch.start();
        let end = rmatch.end();
        let value = rmatch.as_str();

        // If url location contains ':' it's external url, otherwise relative path
        let url = if value.contains(':') {
            Url::parse(value)?
        } else {
            let path = if let Some(rest) = value.strip_prefix('/') {
                // Relative to root path
                root_path.join(rest)
            } else {
                // Relative to current path
                current_path.join(&value)
            };
            let full_path = path.canonicalize().await?;
            Url::from_file_path(full_path)?
        };
        res.html.insert_str(0, &html[end..last_pos]);
        res.html.insert_str(0, &url.to_string());
        res.urls.push(url);
        last_pos = start;
    }
    res.html.insert_str(0, &html[0..last_pos]);

    Ok(res)
}

#[cfg(test)]
mod test_normalize_html_relative_urls {
    use std::io::ErrorKind;

    use super::{normalize_html_relative_urls, Error};

    #[async_std::test]
    async fn test() {
        let html = r#"first
            <!-- Relative to root path -->
            <link href="/style.css" />

            <!-- Relative to current path -->
            <a href="./post02.md">...</a>
            <a href="../pages/example.md">...</a>
            <img src="res01.svg" />

            <!-- Full urls -->
            <a href="https://www.example.com">...</a>
end"#;
        let expect_html = r#"first
            <!-- Relative to root path -->
            <link href="file:///C:/Source/Rust/cianticblog/examples/layout/style.css" />

            <!-- Relative to current path -->
            <a href="file:///C:/Source/Rust/cianticblog/examples/articles/post02.md">...</a>
            <a href="file:///C:/Source/Rust/cianticblog/examples/pages/example.md">...</a>
            <img src="file:///C:/Source/Rust/cianticblog/examples/articles/res01.svg" />

            <!-- Full urls -->
            <a href="https://www.example.com/">...</a>
end"#;

        let value = normalize_html_relative_urls(
            html,
            &"./examples/articles/".into(),
            &"./examples/layout/".into(),
        )
        .await
        .unwrap();
        assert_eq!(value.html, expect_html);
        assert_eq!(value.urls.len(), 5);
    }

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected {} but got {:?}", stringify!($($pattern)+), e),
            }
        }
    }

    #[async_std::test]
    async fn test_urlparsing_failure() {
        let html = r#"
            <a href=":broken">...</a>
        "#;

        assert_err!(
            normalize_html_relative_urls(
                html,
                &"./examples/articles/".into(),
                &"./examples/layout/".into(),
            )
            .await,
            Err(Error::UrlParsingFailed(_))
        );
    }

    #[async_std::test]
    async fn test_canonicalization_failure() {
        let html = r#"
            <a href="/notfound.txt">...</a>
        "#;

        assert_err!(
            normalize_html_relative_urls(
                html,
                &"./examples/articles/".into(),
                &"./examples/layout/".into(),
            )
            .await,
            Err(Error::CanonicalizationFailed(er)) if er.kind() == ErrorKind::NotFound
        );
    }
}
