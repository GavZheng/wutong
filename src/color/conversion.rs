pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn hex_to_rgb(hex: &str) -> String {
    if hex.len() != 7 || !hex.starts_with('#') {
        return "Error: Invalid Hex color format.".to_string();
    }

    // 直接使用字符串切片，而不是字符数组
    let r_str = &hex[1..3];
    let g_str = &hex[3..5];
    let b_str = &hex[5..7];

    // 使用 from_str_radix 转换字符串切片为 u8
    match (
        u8::from_str_radix(r_str, 16),
        u8::from_str_radix(g_str, 16),
        u8::from_str_radix(b_str, 16),
    ) {
        (Ok(r), Ok(g), Ok(b)) => format!("RGB({}, {}, {})", r, g, b),
        _ => "Error: Invalid Hex digits".to_string(),
    }
}

pub fn color_conversion(input: &str) -> [String; 2] {
    // 检查是否是有效的十六进制颜色代码（带 '#' 前缀）
    if input.starts_with('#') && input.len() == 7 && input[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return [hex_to_rgb(input), input.to_string()]
    }

    // 检查是否是有效的 RGB 颜色代码（格式 "RGB(r, g, b)"）
    if input.starts_with("RGB(") && input.ends_with(')') {
        // 提取并解析 RGB 值
        let rgb_values: Vec<u8> = input[4..input.len() - 1]
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        if rgb_values.len() == 3 { 
            return [input.to_string() ,rgb_to_hex(rgb_values[0], rgb_values[1], rgb_values[2])]
        }
    }
    ["Invalid Input".to_string(), "Invalid Input".to_string()]
}