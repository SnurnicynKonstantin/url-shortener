use crate::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Exists { key: String },
    SetWithTTL { key: String, value: String, ttl_secs: u64 },
    SetUrl { url: String },
    SetTTL { key: String, ttl_secs: u64 },
    Info,
    Keys { pattern: Option<String> },
    Clear
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ParseError::Empty);
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_uppercase();
        
        match command.as_str() {
            "SET" => {
                if parts.len() < 3 {
                    return Err(ParseError::InvalidArguments);
                }

                let key = parts[1].to_string();
                let value = parts[2].to_string();
                let mut ttl = None;
                
                if parts.len() >= 5 && parts[3].to_uppercase() == "EX" {
                    ttl = Some(parts[4].parse::<u64>()?);
                }
                
                if let Some(ttl) = ttl {
                    Ok(Command::SetWithTTL { key, value, ttl_secs: ttl })
                } else {
                    Ok(Command::Set { key, value })
                }
            }
            
            "GET" => {
                if parts.len() != 2 {
                    return Err(ParseError::InvalidArguments);
                }
                Ok(Command::Get { key: parts[1].to_string() })
            }
            
            "DEL" | "DELETE" => {
                if parts.len() != 2 {
                    return Err(ParseError::InvalidArguments);
                }
                Ok(Command::Delete { key: parts[1].to_string() })
            }
            
            "EXISTS" => {
                if parts.len() != 2 {
                    return Err(ParseError::InvalidArguments);
                }
                Ok(Command::Exists { key: parts[1].to_string() })
            }

            "CLEAR" => Ok(Command::Clear),
            "INFO" => Ok(Command::Info),
            "KEYS" => {
                let pattern = if parts.len() >= 2 {
                    Some(parts[1].to_string())
                } else {
                    None
                };
                Ok(Command::Keys { pattern })
            }
            
            "SETURL" => {
                if parts.len() != 2 {
                    return Err(ParseError::InvalidArguments);
                }
                Ok(Command::SetUrl { url: parts[1].to_string() })
            }
            
            "SETTTL" => {
                if parts.len() != 3 {
                    return Err(ParseError::InvalidArguments);
                }
                let key = parts[1].to_string();
                let ttl_secs = parts[2].parse::<u64>()?;
                Ok(Command::SetTTL { key, ttl_secs })
            }
            
            _ => Err(ParseError::UnknownCommand(command.to_string())),
        }
    }
}