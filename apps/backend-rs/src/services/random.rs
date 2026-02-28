use base64::{Engine as _, engine::general_purpose};
use rand::{Rng, rngs::ThreadRng};

pub fn generate_token(bytes_len: usize) -> String {
    let mut buffer = vec![0u8; bytes_len];
    ThreadRng::default().fill_bytes(&mut buffer);
    general_purpose::URL_SAFE_NO_PAD.encode(buffer)
}
