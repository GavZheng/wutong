#[cfg(test)]
mod tests {
    use crate::base;

    #[test]
    fn test_base16_decode() {
        assert_eq!(
            base::text::decode::base16_decode("7775746f6e67"),
            "wutong".to_string()
        );
        assert_eq!(
            base::text::decode::base16_decode("5755544f4e47"),
            "WUTONG".to_string()
        );
        assert_eq!(
            base::text::decode::base16_decode("31323334353637383930"),
            "1234567890".to_string()
        );
        assert_eq!(base::text::decode::base16_decode(""), "".to_string());
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(
            base::text::decode::base64_decode("d3V0b25n"),
            "wutong".to_string()
        );
        assert_eq!(
            base::text::decode::base64_decode("V1VUT05H"),
            "WUTONG".to_string()
        );
        assert_eq!(
            base::text::decode::base64_decode("MTIzNDU2Nzg5MA=="),
            "1234567890".to_string()
        );
        assert_eq!(base::text::decode::base64_decode(""), "".to_string());
    }

    #[test]
    fn test_base16_encode() {
        assert_eq!(
            base::text::encode::base16_encode("wutong"),
            "7775746f6e67".to_string()
        );
        assert_eq!(
            base::text::encode::base16_encode("WUTONG"),
            "5755544f4e47".to_string()
        );
        assert_eq!(
            base::text::encode::base16_encode("1234567890"),
            "31323334353637383930".to_string()
        );
        assert_eq!(base::text::encode::base16_encode(""), "".to_string());
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(
            base::text::encode::base64_encode("wutong"),
            "d3V0b25n".to_string()
        );
        assert_eq!(
            base::text::encode::base64_encode("WUTONG"),
            "V1VUT05H".to_string()
        );
        assert_eq!(
            base::text::encode::base64_encode("1234567890"),
            "MTIzNDU2Nzg5MA==".to_string()
        );
        assert_eq!(base::text::encode::base64_encode(""), "".to_string());
    }
}
