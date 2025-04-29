pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> Result<String, &'static str> {
    Ok(format!("#{:02X}{:02X}{:02X}", r, g, b))
}

pub fn hex_to_rgb(hex: &str) -> Result<String, &'static str> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Error: Invalid Hex color format.");
    }

    let r_str = &hex[1..3];
    let g_str = &hex[3..5];
    let b_str = &hex[5..7];

    let r = u8::from_str_radix(r_str, 16).map_err(|_| "Error: Invalid Hex digits")?;
    let g = u8::from_str_radix(g_str, 16).map_err(|_| "Error: Invalid Hex digits")?;
    let b = u8::from_str_radix(b_str, 16).map_err(|_| "Error: Invalid Hex digits")?;

    Ok(format!("RGB({}, {}, {})", r, g, b))
}

pub fn color_conversion(input: &str) -> Result<(String, String), &'static str> {
    if input.starts_with('#')
        && input.len() == 7
        && input[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        let rgb = hex_to_rgb(input)?;
        let hex = input.to_string();
        Ok((rgb, hex))
    } else if input.starts_with("RGB(") && input.ends_with(')') {
        let content = &input[4..input.len() - 1];
        let parts: Vec<&str> = content.split(',').collect();

        if parts.len() != 3 {
            return Err("Error: Invalid RGB format.");
        }

        let mut rgb_values = Vec::new();
        for part in parts {
            let trimmed = part.trim();
            let value = trimmed
                .parse::<u8>()
                .map_err(|_| "Error: Invalid RGB value.")?;
            rgb_values.push(value);
        }

        if rgb_values.len() == 3 {
            let rgb = input.to_string();
            let hex = rgb_to_hex(rgb_values[0], rgb_values[1], rgb_values[2])?;
            Ok((rgb, hex))
        } else {
            Err("Error: Invalid RGB format.")
        }
    } else {
        Err("Error: Invalid Input.")
    }
}
