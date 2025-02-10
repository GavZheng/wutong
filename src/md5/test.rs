#[cfg(test)]
mod tests {
    use crate::md5::text::md5_text;
    #[test]
    fn test_md5() {
        assert_eq!(md5_text("wutong"), "088a63c1ed5b281e11d244ffed22ef37");
    }
}
