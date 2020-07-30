#[derive(Debug)]
pub enum LouisError {
    Io(std::io::Error),
}

impl From<std::io::Error> for LouisError {
    fn from(e: std::io::Error) -> Self {
        LouisError::Io(e)
    }
}

impl std::fmt::Display for LouisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LouisError::Io(e) => write!(f, "{}", e),
        }
    }
}
