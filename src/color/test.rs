#[cfg(test)]
mod tests {
    use crate::color::conversion::{color_conversion, hex_to_rgb, rgb_to_hex};

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(255, 255, 255).unwrap(), "#FFFFFF");
        assert_eq!(rgb_to_hex(0, 0, 0).unwrap(), "#000000");
        assert_eq!(rgb_to_hex(128, 128, 128).unwrap(), "#808080");
        assert_eq!(rgb_to_hex(255, 0, 0).unwrap(), "#FF0000");
        assert_eq!(rgb_to_hex(0, 255, 0).unwrap(), "#00FF00");
    }

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FFFFFF").unwrap(), "RGB(255, 255, 255)");
        assert_eq!(hex_to_rgb("#000000").unwrap(), "RGB(0, 0, 0)");
        assert_eq!(hex_to_rgb("#808080").unwrap(), "RGB(128, 128, 128)");
        assert_eq!(hex_to_rgb("#FF0000").unwrap(), "RGB(255, 0, 0)");
        assert_eq!(hex_to_rgb("#00FF00").unwrap(), "RGB(0, 255, 0)");
    }

    #[test]
    fn test_color_conversion() {
        let input = "#FF0000";
        let result = color_conversion(input).unwrap();
        assert_eq!(result.0, "RGB(255, 0, 0)");
        assert_eq!(result.1, "#FF0000");

        let input = "RGB(255, 0, 0)";
        let result = color_conversion(input).unwrap();
        assert_eq!(result.0, "RGB(255, 0, 0)");
        assert_eq!(result.1, "#FF0000");
    }

    #[test]
    fn test_invalid_input() {
        let input = "RGB(255, 0, 256)";
        assert!(color_conversion(input).is_err());
    }

    #[test]
    fn test_invalid_hex() {
        let input = "#XX0000";
        assert!(hex_to_rgb(input).is_err());
    }

    #[test]
    fn test_invalid_rgb() {
        let input = "RGB(255, 0)";
        assert!(hex_to_rgb(input).is_err());
    }
}
