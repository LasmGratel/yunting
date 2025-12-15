use md5::{Digest, Md5};

pub fn md5hash(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let hash = hasher.finalize();
    base16ct::lower::encode_string(&hash)
}
