#[cfg(test)]
mod tests {
    use crate::md5::text::md5_text;
    #[test]
    fn test_md5() {
        assert_eq!(md5_text("wutong"), "088a63c1ed5b281e11d244ffed22ef37");
    }

    use crate::md5::file::md5_file;
    use crate::Md5Error;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_md5_known_value() -> Result<(), Md5Error> {
        let mut file = NamedTempFile::new()?;
        file.write_all(b"wutong")?;

        let hash = md5_file(file.path())?;

        assert_eq!(hash.trim(), "088a63c1ed5b281e11d244ffed22ef37");

        Ok(())
    }
}
