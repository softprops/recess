//! Linting interfaces

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Request {
    code: String,
}

impl Request {
    /// Returns a new `RequestBuilder` instance configured with code to compile
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
    pub stdout: String,
    pub stderr: String,
}
