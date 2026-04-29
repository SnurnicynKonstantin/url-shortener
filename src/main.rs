use crate::storage::Storage;
use crate::handler::{process_command};
use crate::response::Response;

mod storage;
mod command;
mod response;
mod error;
mod handler;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = Storage::new();

    let commands = vec![
        "SET test hello",
        "SET user:1 john",
        "SET user:2 alice",
        "GET test",
        "GET user:1", 
        "GET nonexistent",
        "EXISTS test",
        "EXISTS missing",
        "SET session:abc token123 EX 5",
        "GET session:abc",
        "SETURL https://example.com",
        "SETURL https://rust-lang.org",
        "KEYS",
        "KEYS user*",
        "DEL test",
        "EXISTS test",
        "INFO",
    ];

    for (i, cmd) in commands.iter().enumerate() {
        println!("\n{}. Команда: {}", i + 1, cmd);
        
        match process_command(&storage, cmd) {
            Ok(response) => {
                print!("   Результат: ");
                print_response(response);
            },
            Err(e) => {
                eprintln!("   Ошибка: {}", e);
            }
        }
    }
    
    Ok(())
}

pub fn print_response(response: Response) {
    match response {
        Response::SimpleString(s) => println!("{}", s),
        Response::Error(e) => eprintln!("ERR {}", e),
        Response::Integer(i) => println!("{}", i),
        Response::BulkString(Some(s)) => println!("\"{}\"", s),
        Response::BulkString(None) => println!("(nil)"),
        Response::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                print!("{}) ", i + 1);
                print_response(item.clone());
            }
        },
        Response::Null => println!("(empty)"),
    }
}