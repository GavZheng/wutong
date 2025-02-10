#[cfg(test)]
mod tests {
    use crate::color::conversion::{hex_to_rgb, rgb_to_hex};

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(255, 255, 255), "#FFFFFF");
        assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
        assert_eq!(rgb_to_hex(128, 128, 128), "#808080");
        assert_eq!(rgb_to_hex(255, 0, 0), "#FF0000");
        assert_eq!(rgb_to_hex(0, 255, 0), "#00FF00");
    }

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FFFFFF"), "RGB(255, 255, 255)");
        assert_eq!(hex_to_rgb("#000000"), "RGB(0, 0, 0)");
        assert_eq!(hex_to_rgb("#808080"), "RGB(128, 128, 128)");
        assert_eq!(hex_to_rgb("#FF0000"), "RGB(255, 0, 0)");
        assert_eq!(hex_to_rgb("#00FF00"), "RGB(0, 255, 0)");
    }
}
