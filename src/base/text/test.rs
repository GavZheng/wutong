#[cfg(test)]
mod tests {
    use crate::base::text::decode::{base16_decode, base64_decode, text_decode};
    use crate::base::text::encode::{base16_encode, base64_encode, text_encode};

    #[test]
    fn test_base16_encode() {
        assert_eq!(base16_encode("wutong").unwrap(), "7775746f6e67");
        assert_eq!(base16_encode("WUTONG").unwrap(), "5755544f4e47");
        assert_eq!(base16_encode("1234567890").unwrap(), "31323334353637383930");
        assert_eq!(base16_encode("").unwrap(), "");
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("wutong").unwrap(), "d3V0b25n");
        assert_eq!(base64_encode("WUTONG").unwrap(), "V1VUT05H");
        assert_eq!(base64_encode("1234567890").unwrap(), "MTIzNDU2Nzg5MA==");
        assert_eq!(base64_encode("").unwrap(), "");
    }

    #[test]
    fn test_base16_decode() {
        assert_eq!(base16_decode("7775746f6e67").unwrap(), "wutong");
        assert_eq!(base16_decode("5755544f4e47").unwrap(), "WUTONG");
        assert_eq!(base16_decode("31323334353637383930").unwrap(), "1234567890");
        assert_eq!(base16_decode("").unwrap(), "");
        assert!(base16_decode("123").is_err());
        assert!(base16_decode("12345").is_err());
        assert!(base16_decode("123G").is_err());
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(base64_decode("d3V0b25n").unwrap(), "wutong");
        assert_eq!(base64_decode("V1VUT05H").unwrap(), "WUTONG");
        assert_eq!(base64_decode("MTIzNDU2Nzg5MA==").unwrap(), "1234567890");
        assert_eq!(base64_decode("").unwrap(), "");
        assert!(base64_decode("ABC").is_err());
        assert!(base64_decode("ABCD==").is_err());
        assert!(base64_decode("ABCD=").is_err());
        assert!(base64_decode("ABCG").is_err());
    }

    #[test]
    fn test_text_encode() {
        let [base16, base64] = text_encode("wutong").unwrap();
        assert_eq!(base16, "7775746f6e67");
        assert_eq!(base64, "d3V0b25n");

        let [base16, base64] = text_encode("WUTONG").unwrap();
        assert_eq!(base16, "5755544f4e47");
        assert_eq!(base64, "V1VUT05H");

        let [base16, base64] = text_encode("1234567890").unwrap();
        assert_eq!(base16, "31323334353637383930");
        assert_eq!(base64, "MTIzNDU2Nzg5MA==");

        let [base16, base64] = text_encode("").unwrap();
        assert_eq!(base16, "");
        assert_eq!(base64, "");
    }

    #[test]
    fn test_text_decode() {
        let [base16_res, base64_res] = text_decode("7775746f6e67");
        assert_eq!(base16_res.unwrap(), "wutong");
        assert!(base64_res.is_err());

        let [base16_res, base64_res] = text_decode("d3V0b25n");
        assert!(base16_res.is_err());
        assert_eq!(base64_res.unwrap(), "wutong");

        let [base16_res, base64_res] = text_decode("5755544f4e47");
        assert_eq!(base16_res.unwrap(), "WUTONG");
        assert!(base64_res.is_err());

        let [base16_res, base64_res] = text_decode("V1VUT05H");
        assert!(base16_res.is_err());
        assert_eq!(base64_res.unwrap(), "WUTONG");

        let [base16_res, base64_res] = text_decode("31323334353637383930");
        assert_eq!(base16_res.unwrap(), "1234567890");
        assert!(base64_res.is_err());

        let [base16_res, base64_res] = text_decode("MTIzNDU2Nzg5MA==");
        assert!(base16_res.is_err());
        assert_eq!(base64_res.unwrap(), "1234567890");

        let [base16_res, base64_res] = text_decode("");
        assert_eq!(base16_res.unwrap(), "");
        assert_eq!(base64_res.unwrap(), "");
    }
}
