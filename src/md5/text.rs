use md5::compute;
pub fn md5_text(input: &str) -> String {
    format!("{:x}", compute(input.as_bytes()))
}
