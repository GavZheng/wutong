pub fn ascii_unicode(input: &str) -> [String; 2] {
    let mut ascii_codes = vec![];
    let mut unicode_escapes = vec![];

    for c in input.chars() {
        let code = c as u32;

        // 转换为ASCII字符串（仅适用于ASCII字符）
        ascii_codes.push(code.to_string());

        // 转换为Unicode格式符，如"\u0077"
        let hex = format!("{:04X}", code);
        unicode_escapes.push(format!(r"\u{}", hex.to_lowercase()));
    }

    [ascii_codes.join(" "), unicode_escapes.join("")]
}
