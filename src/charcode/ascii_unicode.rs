pub fn ascii_unicode(input: &str) -> Result<[String; 2], String> {
    let mut ascii_codes = Vec::new();
    let mut unicode_escapes = Vec::new();

    for c in input.chars() {
        let code = c as u32;
        ascii_codes.push(code.to_string());
        let hex = format!("{:04X}", code);
        let unicode_escape = format!(r"\u{}", hex.to_lowercase());
        unicode_escapes.push(unicode_escape);
    }

    Ok([ascii_codes.join(" "), unicode_escapes.join("")])
}
