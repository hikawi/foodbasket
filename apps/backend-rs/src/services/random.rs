use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rand::Rng;

pub fn generate_random_token(length: usize) -> String {
    let mut bytes = vec![0u8; length];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE.encode(bytes)
}
