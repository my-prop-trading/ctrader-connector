pub fn generate_password_hash(src: &str) -> String {
    format!("{:x}", md5::compute(src.as_bytes()))
}