#[derive(Debug, Clone)]
pub enum Response {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Vec<Response>),
    Null,
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Response::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            Response::Error(e) => format!("-ERR {}\r\n", e).into_bytes(),
            Response::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            Response::BulkString(Some(s)) => format!("${}\r\n{}\r\n", s.len(), s).into_bytes(),
            Response::BulkString(None) => "$-1\r\n".to_string().into_bytes(),
            Response::Array(arr) => {
                let mut result = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    result.extend(item.to_bytes());
                }
                result
            }
            Response::Null => "".to_string().into_bytes(),
        }
    }
    
    pub fn ok() -> Self {
        Response::SimpleString("OK".to_string())
    }
    
    pub fn error(msg: impl ToString) -> Self {
        Response::Error(msg.to_string())
    }
}