use crate::storage::Storage;
use crate::handler::process_command;
use crate::response::Response;
use std::thread;
use std::time::Duration;

#[test]
fn test_expire_sets_ttl_for_existing_key() {
    let storage = Storage::new();
    
    process_command(&storage, "SET test_key test_value").unwrap();
    
    let result = process_command(&storage, "SETTTL test_key 5").unwrap();
    assert_eq!(result, Response::Integer(1));
    
    let get_result = process_command(&storage, "GET test_key").unwrap();
    assert_eq!(get_result, Response::BulkString(Some("test_value".to_string())));
}

#[test]
fn test_expire_returns_zero_for_nonexistent_key() {
    let storage = Storage::new();
    
    let result = process_command(&storage, "SETTTL nonexistent_key 5").unwrap();
    assert_eq!(result, Response::Integer(0));
}

#[test]
fn test_expire_returns_zero_for_expired_key() {
    let storage = Storage::new();
    
    process_command(&storage, "SET test_key test_value EX 1").unwrap();
    
    thread::sleep(Duration::from_secs(2));
    
    let result = process_command(&storage, "SETTTL test_key 5").unwrap();
    assert_eq!(result, Response::Integer(0));
}

#[test]
fn test_expire_overwrites_existing_ttl() {
    let storage = Storage::new();
    
    process_command(&storage, "SET test_key test_value EX 10").unwrap();
    
    let result = process_command(&storage, "SETTTL test_key 2").unwrap();
    assert_eq!(result, Response::Integer(1));
    
    let get_result = process_command(&storage, "GET test_key").unwrap();
    assert_eq!(get_result, Response::BulkString(Some("test_value".to_string())));
    
    thread::sleep(Duration::from_secs(3));
    
    let expired_result = process_command(&storage, "GET test_key").unwrap();
    assert_eq!(expired_result, Response::BulkString(None));
}

#[test]
fn test_expire_key_becomes_inaccessible_after_ttl() {
    let storage = Storage::new();
    
    process_command(&storage, "SET short_ttl_key test_value").unwrap();
    process_command(&storage, "SETTTL short_ttl_key 1").unwrap();
    
    let before_expire = process_command(&storage, "GET short_ttl_key").unwrap();
    assert_eq!(before_expire, Response::BulkString(Some("test_value".to_string())));
    
    thread::sleep(Duration::from_secs(2));
    
    let after_expire = process_command(&storage, "GET short_ttl_key").unwrap();
    assert_eq!(after_expire, Response::BulkString(None));
}

#[test]
fn test_expire_command_parsing() {
    let storage = Storage::new();
    
    process_command(&storage, "SET key1 value1").unwrap();
    
    let result1 = process_command(&storage, "SETTTL key1 30").unwrap();
    assert_eq!(result1, Response::Integer(1));
    
    let result2 = process_command(&storage, "SETTTL key1 0").unwrap();
    assert_eq!(result2, Response::Integer(1));
    
    let get_result_after_zero = process_command(&storage, "GET key1").unwrap();
    assert_eq!(get_result_after_zero, Response::BulkString(None));
    
    process_command(&storage, "SET key2 value2").unwrap();
    let result3 = process_command(&storage, "SETTTL key2 999999").unwrap();
    assert_eq!(result3, Response::Integer(1));
}

#[test]
fn test_expire_command_parsing_errors() {
    let storage = Storage::new();
    
    let result1 = process_command(&storage, "SETTTL");
    assert!(result1.is_err());
    
    let result2 = process_command(&storage, "SETTTL key1");
    assert!(result2.is_err());
    
    let result3 = process_command(&storage, "SETTTL key1 not_a_number");
    assert!(result3.is_err());
    
    let result4 = process_command(&storage, "SETTTL key1 30 extra_arg");
    assert!(result4.is_err());
}