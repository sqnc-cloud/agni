pub enum Command {
    Ping,
    Unknown(String),
}

impl Command {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let input = String::from_utf8_lossy(bytes);
        let command = input.split_whitespace().next().unwrap_or("").to_uppercase();

        match command.as_str() {
            "PING" => Command::Ping,
            _ => Command::Unknown(command),
        }
    }
}

pub enum Response {
    Pong,
    Error(String),
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Response::Pong => b"PONG".to_vec(),
            Response::Error(msg) => format!("ERR {}", msg).into_bytes(),
        }
    }
}
