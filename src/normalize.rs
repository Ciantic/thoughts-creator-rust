use normpath::PathExt;

/// Normalize
///
/// Works on windows, see https://github.com/rust-lang/rust/issues/80884
pub async fn normalize(
    path: &async_std::path::PathBuf,
) -> Result<async_std::path::PathBuf, std::io::Error> {
    let b: std::path::PathBuf = std::path::PathBuf::from(path).normalize()?.into();
    Ok(b.into())
    /*
    let res = path.canonicalize().await?;

    if cfg!(windows) {
        Ok(async_std::path::PathBuf::from(
            res.to_string_lossy().trim_start_matches(r"\\?\"),
        ))
    } else {
        Ok(res)
    }
     */
}

pub fn normalize_sync(path: &std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    Ok(path.normalize()?.into())
    /*
    let res = path.canonicalize()?;
    if cfg!(windows) {
        Ok(std::path::PathBuf::from(
            res.to_string_lossy().trim_start_matches(r"\\?\"),
        ))
    } else {
        Ok(res)
    }
     */
}

#[cfg(test)]
mod test {
    use super::{normalize, normalize_sync};

    #[async_std::test]
    async fn test_normalize() {
        use async_std::path::PathBuf;
        let cwindows: PathBuf = "C:\\Windows\\".into();
        let cwindows_can = normalize(&cwindows.clone()).await.unwrap();
        assert_eq!(cwindows, cwindows_can);
    }

    #[test]
    fn test_normalize_sync() {
        use std::path::PathBuf;
        let cwindows: PathBuf = "C:\\Windows\\".into();
        let cwindows_can = normalize_sync(&cwindows).unwrap();
        assert_eq!(cwindows, cwindows_can);
    }
}
