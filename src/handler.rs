use crate::storage::Storage;
use crate::command::Command;
use crate::response::Response;

pub fn process_command(storage: &Storage, input: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let command = Command::parse(input)?;
    
    let response = match command {
        Command::Set { key, value } => storage.set(key, value)?,
        Command::SetEx { key, value, ttl_secs } => storage.set_ex(key, value, ttl_secs)?,
        Command::Get { key } => storage.get(&key)?,
        Command::Delete { key } => storage.delete(&key)?,
        Command::Exists { key } => storage.exists(&key)?,
        Command::Keys { pattern } => {
            let pattern = pattern.as_deref();
            storage.keys(pattern)?
        },
        Command::Clear => storage.clear()?,
        Command::Info => storage.info()?,
    };

    Ok(response)
}
