use async_std::path::{Path, PathBuf};
use chrono::{DateTime, TimeZone, Utc};
use std::{
    ffi::OsStr,
    path::{Component, Components},
    process::{Command, Stdio},
};
#[derive(Debug)]
pub enum Error {
    GitDateParseFailure,
    IOError(std::io::Error),
}

trait MaybeArg {
    fn arg_if<S: AsRef<OsStr>>(&mut self, arg: Option<S>) -> &mut Self;
}

impl MaybeArg for std::process::Command {
    fn arg_if<S: AsRef<OsStr>>(&mut self, arg: Option<S>) -> &mut Command {
        if let Some(a) = arg {
            self.arg(a)
        } else {
            self
        }
    }
}

async fn git_date(file: &PathBuf, flag: Option<&str>) -> Result<DateTime<Utc>, Error> {
    let file = file.canonicalize().await.map_err(Error::IOError)?;

    // After canonicalization the path is guaranteed to have filename and dirname
    let filename = file.file_name().unwrap();
    let dirname = file.parent().unwrap();

    let mut cmd = Command::new("git");
    cmd.current_dir(dirname)
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%ci")
        .arg_if(flag)
        .arg(filename)
        .stdout(Stdio::piped()) // redirect the stdout
        .stderr(Stdio::piped()); // redirect the stderr;

    let out = cmd.output().map_err(Error::IOError)?;
    let out_str = String::from_utf8_lossy(&out.stdout);
    DateTime::parse_from_str(&out_str, "%Y-%m-%d %H:%M:%S %z")
        .map_err(|_| Error::GitDateParseFailure)
        .map(|d| d.into())
}

pub async fn git_created(file: &PathBuf) -> Result<DateTime<Utc>, Error> {
    git_date(file, Some("--diff-filter=A")).await
}

pub async fn git_modified(file: &PathBuf) -> Result<DateTime<Utc>, Error> {
    git_date(file, None).await
}

#[cfg(test)]
mod test_git_date {
    use super::{git_created, git_modified};
    use chrono::{DateTime, TimeZone, Utc};

    #[async_std::test]
    async fn test_git_created() {
        let created = git_created(&"./examples/articles/post01.md".into())
            .await
            .unwrap();
        assert_eq!(created, Utc.ymd(2021, 1, 1).and_hms(20, 56, 55));
    }

    #[async_std::test]
    async fn test_git_modified() {
        let created = git_modified(&"./examples/articles/post01.md".into())
            .await
            .unwrap();
        assert_eq!(created, Utc.ymd(2021, 1, 3).and_hms(12, 42, 37));
    }
}
