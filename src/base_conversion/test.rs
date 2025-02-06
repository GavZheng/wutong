#[cfg(test)]
mod tests {
    use crate::base_conversion;
    
    #[test]
    fn test_math_base_conversion() {
        assert_eq!(base_conversion::math::binary("11000"), ["30", "24", "18"]);
        assert_eq!(base_conversion::math::decimal("24"), ["11000", "30", "18"]);
        assert_eq!(base_conversion::math::octal("30"), ["11000", "24", "18"]);
        assert_eq!(base_conversion::math::hexadecimal("18"), ["11000", "30", "24"]);
    }
}