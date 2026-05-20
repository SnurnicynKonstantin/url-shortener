use std::hash::{Hash, Hasher};
use fnv::FnvHasher;
use base64ct::{Base64UrlUnpadded, Encoding};

pub fn generate_short_url(url: &str) -> String {
    let mut hasher = FnvHasher::default();
    url.hash(&mut hasher);
    let hash = hasher.finish();

    let hash_bytes = hash.to_be_bytes();
    
    let encoded = Base64UrlUnpadded::encode_string(&hash_bytes);
    
    encoded.chars().take(6).collect()
}