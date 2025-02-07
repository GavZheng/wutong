#[cfg(test)]
mod tests {
    use crate::charcode;

    #[test]
    fn test_ascii_unicode() {
        assert_eq!(
            charcode::ascii_unicode::ascii_unicode("wutong"),
            [
                "119 117 116 111 110 103",
                r"\u0077\u0075\u0074\u006f\u006e\u0067"
            ]
        );
    }
}
