// pub fn windows_remove_unc(path: &std::path::PathBuf) -> std::path::PathBuf {
// let mut coms = path.components();
// if let Some(first) = coms.next() {
//     if let std::path::Component::Prefix(p) = first {
//         if let Some(str) = p.as_os_str().to_string_lossy().strip_prefix(r"\\?\") {

//         }
//         println!("{:?}", p);
//     }
// }
// }

/// Normalize
///
/// Works on windows, see https://github.com/rust-lang/rust/issues/80884
pub async fn normalize(
    path: &async_std::path::PathBuf,
) -> Result<async_std::path::PathBuf, std::io::Error> {
    let res = path.canonicalize().await?;

    if cfg!(windows) {
        Ok(async_std::path::PathBuf::from(
            res.to_string_lossy().trim_start_matches(r"\\?\"),
        ))
    } else {
        Ok(res)
    }
}

pub fn normalize_sync(path: &std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    let res = path.canonicalize()?;
    if cfg!(windows) {
        Ok(std::path::PathBuf::from(
            res.to_string_lossy().trim_start_matches(r"\\?\"),
        ))
    } else {
        Ok(res)
    }
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
