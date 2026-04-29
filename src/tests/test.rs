#[cfg(test)]
mod tests {
    use crate::storage::Storage;
    use crate::handler::process_command;
    use crate::response::Response;

    fn create_storage() -> Storage {
        Storage::new()
    }

    #[test]
    fn test_set_and_get_commands() {
        let storage = create_storage();

        let set_result = process_command(&storage, "SET test hello").unwrap();
        match set_result {
            Response::SimpleString(s) => assert_eq!(s, "OK"),
            _ => panic!("Expected SimpleString(OK), got {:?}", set_result),
        }

        let get_result = process_command(&storage, "GET test").unwrap();
        match get_result {
            Response::BulkString(Some(s)) => assert_eq!(s, "hello"),
            _ => panic!("Expected BulkString(Some(hello)), got {:?}", get_result),
        }

        let get_missing_result = process_command(&storage, "GET missing").unwrap();
        match get_missing_result {
            Response::BulkString(None) => {},
            _ => panic!("Expected BulkString(None), got {:?}", get_missing_result),
        }
    }

    #[test]
    fn test_exists_command() {
        let storage = create_storage();

        process_command(&storage, "SET user:1 alice").unwrap();

        let exists_result = process_command(&storage, "EXISTS user:1").unwrap();
        match exists_result {
            Response::Integer(1) => {},
            _ => panic!("Expected Integer(1), got {:?}", exists_result),
        }

        let not_exists_result = process_command(&storage, "EXISTS missing").unwrap();
        match not_exists_result {
            Response::Integer(0) => {},
            _ => panic!("Expected Integer(0), got {:?}", not_exists_result),
        }
    }

    #[test]
    fn test_delete_command() {
        let storage = create_storage();

        process_command(&storage, "SET temp value").unwrap();

        let exists_before = process_command(&storage, "EXISTS temp").unwrap();
        match exists_before {
            Response::Integer(1) => {},
            _ => panic!("Key should exist before deletion"),
        }

        let delete_result = process_command(&storage, "DEL temp").unwrap();
        match delete_result {
            Response::Integer(1) => {},
            _ => panic!("Expected Integer(1) for successful deletion, got {:?}", delete_result),
        }

        let exists_after = process_command(&storage, "EXISTS temp").unwrap();
        match exists_after {
            Response::Integer(0) => {},
            _ => panic!("Key should not exist after deletion"),
        }

        let delete_missing = process_command(&storage, "DEL missing").unwrap();
        match delete_missing {
            Response::Integer(0) => {},
            _ => panic!("Expected Integer(0) for deleting non-existing key, got {:?}", delete_missing),
        }
    }

    #[test]
    fn test_set_with_expiration() {
        let storage = create_storage();

        let set_ex_result = process_command(&storage, "SET session:abc token123 EX 60").unwrap();
        match set_ex_result {
            Response::SimpleString(s) => assert_eq!(s, "OK"),
            _ => panic!("Expected SimpleString(OK), got {:?}", set_ex_result),
        }

        let get_result = process_command(&storage, "GET session:abc").unwrap();
        match get_result {
            Response::BulkString(Some(s)) => assert_eq!(s, "token123"),
            _ => panic!("Expected BulkString(Some(token123)), got {:?}", get_result),
        }
    }

    #[test]
    fn test_keys_command() {
        let storage = create_storage();

        process_command(&storage, "SET user:1 alice").unwrap();
        process_command(&storage, "SET user:2 bob").unwrap();
        process_command(&storage, "SET config:1 value").unwrap();

        let all_keys_result = process_command(&storage, "KEYS").unwrap();
        match all_keys_result {
            Response::Array(keys) => {
                assert_eq!(keys.len(), 3);
            },
            _ => panic!("Expected Array response, got {:?}", all_keys_result),
        }

        let user_keys_result = process_command(&storage, "KEYS user*").unwrap();
        match user_keys_result {
            Response::Array(keys) => {
                assert_eq!(keys.len(), 2);
                for key in keys {
                    match key {
                        Response::BulkString(Some(k)) => assert!(k.contains("user")),
                        _ => panic!("Expected BulkString in array"),
                    }
                }
            },
            _ => panic!("Expected Array response, got {:?}", user_keys_result),
        }
    }

    #[test]
    fn test_clear_command() {
        let storage = create_storage();

        process_command(&storage, "SET key1 value1").unwrap();
        process_command(&storage, "SET key2 value2").unwrap();

        let keys_before = process_command(&storage, "KEYS").unwrap();
        match keys_before {
            Response::Array(keys) => assert_eq!(keys.len(), 2),
            _ => panic!("Expected Array response"),
        }

        let flush_result = process_command(&storage, "CLEAR").unwrap();
        match flush_result {
            Response::SimpleString(s) => assert_eq!(s, "OK"),
            _ => panic!("Expected SimpleString(OK), got {:?}", flush_result),
        }

        let keys_after = process_command(&storage, "KEYS").unwrap();
        match keys_after {
            Response::Array(keys) => assert_eq!(keys.len(), 0),
            _ => panic!("Expected empty Array response"),
        }
    }

    #[test]
    fn test_info_command() {
        let storage = create_storage();

        process_command(&storage, "SET test1 value1").unwrap();
        process_command(&storage, "SET test2 value2").unwrap();
        process_command(&storage, "GET test1").unwrap();
        process_command(&storage, "GET missing").unwrap();
        
        let info_result = process_command(&storage, "INFO").unwrap();
        match info_result {
            Response::BulkString(Some(info)) => {
                assert!(info.contains("Stats:"));
                assert!(info.contains("total_keys: 2"));
            },
            _ => panic!("Expected BulkString with info, got {:?}", info_result),
        }
    }

    #[test]
    fn test_invalid_command() {
        let storage = create_storage();
        
        let result = process_command(&storage, "INVALID_COMMAND");
        assert!(result.is_err(), "Expected error for invalid command");
        
        let result = process_command(&storage, "SET");
        assert!(result.is_err(), "Expected error for incomplete SET command");
        
        let result = process_command(&storage, "GET");
        assert!(result.is_err(), "Expected error for incomplete GET command");
    }

    #[test]
    fn test_empty_command() {
        let storage = create_storage();
        
        let result = process_command(&storage, "");
        assert!(result.is_err(), "Expected error for empty command");
        
        let result = process_command(&storage, "   ");
        assert!(result.is_err(), "Expected error for whitespace-only command");
    }

    #[test]
    fn test_seturl_command() {
        let storage = create_storage();

        let seturl_result = process_command(&storage, "SETURL https://example.com").unwrap();
        match seturl_result {
            Response::BulkString(Some(short_key)) => {
                assert!(!short_key.is_empty(), "Short key should not be empty");
                
                let get_result = process_command(&storage, &format!("GET {}", short_key)).unwrap();
                match get_result {
                    Response::BulkString(Some(url)) => assert_eq!(url, "https://example.com"),
                    _ => panic!("Expected to retrieve original URL, got {:?}", get_result),
                }
            },
            _ => panic!("Expected BulkString with short key, got {:?}", seturl_result),
        }
    }

    #[test]
    fn test_seturl_generates_consistent_keys() {
        let storage = create_storage();

        let result1 = process_command(&storage, "SETURL https://rust-lang.org").unwrap();
        let result2 = process_command(&storage, "SETURL https://rust-lang.org").unwrap();

        match (result1, result2) {
            (Response::BulkString(Some(key1)), Response::BulkString(Some(key2))) => {
                assert_eq!(key1, key2, "Same URL should generate the same short key");
            },
            _ => panic!("Expected BulkString responses from both SETURL commands"),
        }
    }

    #[test]
    fn test_seturl_different_keys_for_different_urls() {
        let storage = create_storage();

        let result1 = process_command(&storage, "SETURL https://example.com").unwrap();
        let result2 = process_command(&storage, "SETURL https://google.com").unwrap();

        match (result1, result2) {
            (Response::BulkString(Some(key1)), Response::BulkString(Some(key2))) => {
                assert_ne!(key1, key2, "Different URLs should generate different short keys");
            },
            _ => panic!("Expected BulkString responses from both SETURL commands"),
        }
    }

    #[test]
    fn test_seturl_invalid_arguments() {
        let storage = create_storage();
        
        let result = process_command(&storage, "SETURL");
        assert!(result.is_err(), "Expected error for SETURL without URL argument");
        
        let result = process_command(&storage, "SETURL url1 url2");
        assert!(result.is_err(), "Expected error for SETURL with too many arguments");
    }
}