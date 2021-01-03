use async_std::path::{Path, PathBuf};
use chrono::{DateTime, TimeZone, Utc};
use std::{
    ffi::OsStr,
    path::{Component, Components},
    process::Command,
};
#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
}

pub async fn git_date(file: &PathBuf, flag: String) -> Result<DateTime<Utc>, Error> {
    let file = file.canonicalize().await.map_err(Error::IOError)?;

    // let filename = com.last();
    // let dirname = com.as_path();
    let cmd = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg(flag)
        .arg("--pretty=format:%ci")
        .arg(file.file_name().unwrap_or(OsStr::new("")));

    todo!()
}

fn split_dir_and_file_name(file: &PathBuf) -> (Option<&Path>, Option<&OsStr>) {
    (file.parent(), file.file_name())

    // If there is a way, then there is a determination, this is funny:
    //
    // file.iter().rev().fold((None, None), |df, item| match df {
    //     (None, None) => (None, Some(item.into())),
    //     (Some(d), f) => (Some(PathBuf::from(item).join(d)), f),
    //     (None, f) => (Some(PathBuf::new().join(item)), f),
    // })
}

#[cfg(test)]
mod test_frontmatter {
    use async_std::path::PathBuf;

    use super::split_dir_and_file_name;

    #[async_std::test]
    async fn test_split_filename() {
        let filepath = PathBuf::from(&"./examples/articles/post01.md")
            .canonicalize()
            .await
            .unwrap();
        let (a, b) = split_dir_and_file_name(&filepath);
        println!("{:?} {:?}", a, b);
    }
}
