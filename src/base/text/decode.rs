pub fn base16_decode(input: &str) -> String {
    const BASE16_CHARS: &str = "0123456789abcdef";
    let mut char_to_index = [255; 256];
    for (i, c) in BASE16_CHARS.chars().enumerate() {
        char_to_index[c as usize] = (i % 16) as u8;
    }

    if input.len() % 2 != 0 {
        return String::from("Invalid base16 input: length is not even");
    }

    let mut result = Vec::new();

    for (i, chunk) in input.as_bytes().chunks(2).enumerate() {
        if chunk.len() != 2 {
            return format!(
                "Invalid base16 input: chunk at index {} has invalid length",
                i * 2
            );
        }

        let high_nibble = match char_to_index[chunk[0] as usize] {
            0..=15 => char_to_index[chunk[0] as usize],
            _ => {
                return format!(
                    "Invalid base16 input: invalid character '{}' at index {}",
                    char::from(chunk[0]),
                    i * 2
                )
            }
        };

        let low_nibble = match char_to_index[chunk[1] as usize] {
            0..=15 => char_to_index[chunk[1] as usize],
            _ => {
                return format!(
                    "Invalid base16 input: invalid character '{}' at index {}",
                    char::from(chunk[1]),
                    i * 2 + 1
                )
            }
        };

        let byte = (high_nibble << 4) | low_nibble;
        result.push(byte);
    }

    String::from_utf8(result)
        .unwrap_or_else(|_| String::from("Invalid UTF-8 sequence in decoded data"))
}

pub fn base64_decode(input: &str) -> String {
    const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();

    if input.len() % 4 != 0 {
        return String::from("Invalid Base64 input.");
    }

    let chunks: Vec<_> = input.as_bytes().chunks(4).collect();

    for (chunk_idx, chunk) in chunks.iter().enumerate() {
        let is_last_chunk = chunk_idx == chunks.len() - 1;

        if chunk.len() != 4 {
            return String::from("Invalid Base64 input.");
        }

        let mut values = [0u8; 4];
        let mut pad_count = 0;

        for (i, &byte) in chunk.iter().enumerate() {
            let c = byte as char;

            if c == '=' {
                if !is_last_chunk || i < 2 {
                    return String::from("Invalid Base64 input.");
                }
                pad_count += 1;
                values[i] = 0;
            } else {
                match BASE64_CHARS.find(c) {
                    Some(idx) => values[i] = idx as u8,
                    None => return String::from("Invalid Base64 input."),
                }

                if is_last_chunk && pad_count > 0 {
                    return String::from("Invalid Base64 input.");
                }
            }
        }

        if pad_count > 2 {
            return String::from("Invalid Base64 input.");
        }

        if is_last_chunk {
            match pad_count {
                1 if chunk[3] != b'=' => return String::from("Invalid Base64 input."),
                2 if chunk[2] != b'=' || chunk[3] != b'=' => {
                    return String::from("Invalid Base64 input.")
                }
                _ => (),
            }
        }

        let b0 = (values[0] << 2) | (values[1] >> 4);
        let b1 = ((values[1] & 0x0F) << 4) | (values[2] >> 2);
        let b2 = ((values[2] & 0x03) << 6) | values[3];

        let bytes_to_decode = 3 - pad_count;
        match bytes_to_decode {
            3 => result.extend_from_slice(&[b0, b1, b2]),
            2 => result.extend_from_slice(&[b0, b1]),
            1 => result.push(b0),
            _ => unreachable!(),
        }
    }

    String::from_utf8(result)
        .unwrap_or_else(|_| String::from("Invalid UTF-8 sequence in decoded data"))
}

pub fn text_decode(input: &str) -> [String; 2] {
    let base16 = base16_decode(input);
    let base64 = base64_decode(input);
    [base16, base64]
}
