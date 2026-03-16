// imgconv - 중복 파일 건너뛰기 모듈

#![cfg(feature = "dedup")]

use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::error::ConvertError;

/// 파일의 SHA-256 해시 계산 (hex-encoded 문자열 반환)
pub fn file_hash(path: &Path) -> Result<String, ConvertError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// 두 파일의 해시를 비교하여 동일 여부 반환
pub fn is_identical(path_a: &Path, path_b: &Path) -> Result<bool, ConvertError> {
    let hash_a = file_hash(path_a)?;
    let hash_b = file_hash(path_b)?;
    Ok(hash_a == hash_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_hash_deterministic() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(b"hello world").unwrap();
        tmp.flush().unwrap();

        let h1 = file_hash(tmp.path()).unwrap();
        let h2 = file_hash(tmp.path()).unwrap();
        assert_eq!(h1, h2);
        // Known SHA-256 of "hello world"
        assert_eq!(
            h1,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_is_identical_same_content() {
        let mut a = NamedTempFile::new().unwrap();
        let mut b = NamedTempFile::new().unwrap();
        a.write_all(b"identical content").unwrap();
        b.write_all(b"identical content").unwrap();
        a.flush().unwrap();
        b.flush().unwrap();

        assert!(is_identical(a.path(), b.path()).unwrap());
    }

    #[test]
    fn test_is_identical_different_content() {
        let mut a = NamedTempFile::new().unwrap();
        let mut b = NamedTempFile::new().unwrap();
        a.write_all(b"content a").unwrap();
        b.write_all(b"content b").unwrap();
        a.flush().unwrap();
        b.flush().unwrap();

        assert!(!is_identical(a.path(), b.path()).unwrap());
    }

    #[test]
    fn test_file_hash_empty_file() {
        let tmp = NamedTempFile::new().unwrap();
        let hash = file_hash(tmp.path()).unwrap();
        // Known SHA-256 of empty input
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_file_hash_nonexistent_file() {
        let result = file_hash(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Property 26: SHA-256 해시 일관성 및 중복 감지
    // — 같은 파일 해시 두 번 계산 시 동일, 동일 파일 쌍 `is_identical` = true
    // **Validates: Requirements 34.1, 34.2**

    proptest! {
        #[test]
        fn prop26_hash_consistency(content in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let mut tmp = NamedTempFile::new().unwrap();
            tmp.write_all(&content).unwrap();
            tmp.flush().unwrap();

            let h1 = file_hash(tmp.path()).unwrap();
            let h2 = file_hash(tmp.path()).unwrap();
            prop_assert_eq!(&h1, &h2, "Same file must produce identical hashes");
        }
    }

    proptest! {
        #[test]
        fn prop26_identical_files_detected(content in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let mut a = NamedTempFile::new().unwrap();
            let mut b = NamedTempFile::new().unwrap();
            a.write_all(&content).unwrap();
            b.write_all(&content).unwrap();
            a.flush().unwrap();
            b.flush().unwrap();

            prop_assert!(
                is_identical(a.path(), b.path()).unwrap(),
                "Two files with identical content must be detected as identical"
            );
        }
    }

    proptest! {
        #[test]
        fn prop26_different_files_not_identical(
            content_a in proptest::collection::vec(any::<u8>(), 1..512),
            content_b in proptest::collection::vec(any::<u8>(), 1..512),
        ) {
            prop_assume!(content_a != content_b);

            let mut a = NamedTempFile::new().unwrap();
            let mut b = NamedTempFile::new().unwrap();
            a.write_all(&content_a).unwrap();
            b.write_all(&content_b).unwrap();
            a.flush().unwrap();
            b.flush().unwrap();

            prop_assert!(
                !is_identical(a.path(), b.path()).unwrap(),
                "Two files with different content must not be detected as identical"
            );
        }
    }
}

