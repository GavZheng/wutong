#[cfg(test)]
mod tests {
    use crate::base_conversion::math;

    #[test]
    fn test_math_base_conversion() {
        // Test binary conversion
        let binary_result = math::binary("11000").unwrap();
        assert_eq!(binary_result, ["30", "24", "18"]);

        // Test decimal conversion
        let decimal_result = math::decimal("24").unwrap();
        assert_eq!(decimal_result, ["11000", "30", "18"]);

        // Test octal conversion
        let octal_result = math::octal("30").unwrap();
        assert_eq!(octal_result, ["11000", "24", "18"]);

        // Test hexadecimal conversion
        let hexadecimal_result = math::hexadecimal("18").unwrap();
        assert_eq!(hexadecimal_result, ["11000", "30", "24"]);
    }
}
