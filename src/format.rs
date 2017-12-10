//! Format interfaces

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Request {
    code: String,
}

impl Request {
    pub fn new<C>(code: C) -> Self
    where
        C: Into<String>,
    {
        Request { code: code.into() }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}