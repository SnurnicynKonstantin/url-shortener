use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn generate_short_url(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();

    let truncated_hash = (hash & 0xFFFFFFFF) as u32;
    
    let base62_chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut result = String::new();
    let mut n = truncated_hash;

    while n > 0 && result.len() < 6 {
        result.push(base62_chars.chars().nth((n % 62) as usize).unwrap());
        n /= 62;
    }
    
    if result.is_empty() {
        result.push('0');
    }
    
    result.chars().rev().collect()
}