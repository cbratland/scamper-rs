#[derive(Debug, Clone, Copy)]
pub struct ErrorCode(u32);

impl ErrorCode {
    pub fn new(code: u32) -> Self {
        Self(code)
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{:04}", self.0)
    }
}
