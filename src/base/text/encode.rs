use std::string::FromUtf8Error;

pub fn base16_encode(input: &str) -> Result<String, FromUtf8Error> {
    const BASE16_CHARS: &[u8] = b"0123456789abcdef";

    let bytes = input.as_bytes();
    let mut result = Vec::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        result.push(BASE16_CHARS[(byte >> 4) as usize]);
        result.push(BASE16_CHARS[(byte & 0x0F) as usize]);
    }

    String::from_utf8(result)
}

pub fn base64_encode(input: &str) -> Result<String, FromUtf8Error> {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = Vec::with_capacity(((bytes.len() + 2) / 3) * 4);
    let mut buffer = [0u8; 3];
    let mut buffer_index = 0;

    for byte in bytes {
        buffer[buffer_index] = *byte;
        buffer_index += 1;

        if buffer_index == 3 {
            result.push(BASE64_CHARS[((buffer[0] & 0xFC) >> 2) as usize]);
            result.push(
                BASE64_CHARS[(((buffer[0] & 0x03) << 4) | ((buffer[1] & 0xF0) >> 4)) as usize],
            );
            result.push(
                BASE64_CHARS[(((buffer[1] & 0x0F) << 2) | ((buffer[2] & 0xC0) >> 6)) as usize],
            );
            result.push(BASE64_CHARS[(buffer[2] & 0x3F) as usize]);
            buffer_index = 0;
        }
    }

    if buffer_index > 0 {
        result.push(BASE64_CHARS[((buffer[0] & 0xFC) >> 2) as usize]);

        if buffer_index == 1 {
            result.push(BASE64_CHARS[((buffer[0] & 0x03) << 4) as usize]);
            result.push(b'=');
            result.push(b'=');
        } else {
            result.push(
                BASE64_CHARS[(((buffer[0] & 0x03) << 4) | ((buffer[1] & 0xF0) >> 4)) as usize],
            );
            result.push(BASE64_CHARS[((buffer[1] & 0x0F) << 2) as usize]);
            result.push(b'=');
        }
    }

    String::from_utf8(result)
}

pub fn text_encode(input: &str) -> Result<[String; 2], FromUtf8Error> {
    let base16 = base16_encode(input)?;
    let base64 = base64_encode(input)?;
    Ok([base16, base64])
}
