#[derive(Debug)]
pub enum ClientError {
    Internal(String),
    User(String),
}

pub fn internal(desc: &str) -> ClientError {
    ClientError::Internal(desc.to_string())
}

pub fn user(desc: &str) -> ClientError {
    ClientError::User(desc.to_string())
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClientError::Internal(desc) => {
                    format!("\x1b[91mInternal error:\x1b[m {}", desc)
                }
                ClientError::User(desc) => desc.to_string(),
            }
        )
    }
}
