#[cfg(test)]
mod tests {
    #[test]
    fn test_ascii_unicode() {
        use crate::charcode::ascii_unicode::ascii_unicode;

        let expected = [
            "119 117 116 111 110 103".to_string(),
            r"\u0077\u0075\u0074\u006f\u006e\u0067".to_string(),
        ];
        assert_eq!(ascii_unicode("wutong").unwrap(), expected);
    }
}
