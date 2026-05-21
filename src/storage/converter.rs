use base64ct::{Base64UrlUnpadded, Encoding};

pub fn generate_short_url(url: &str) -> String {
    let hash = blake3::hash(url.as_bytes());
    let hash_bytes = hash.as_bytes();
    
    let encoded = Base64UrlUnpadded::encode_string(&hash_bytes[..6]);
    
    encoded
}