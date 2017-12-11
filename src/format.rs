//! Format interfaces

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Request {
    code: String,
}

impl Request {
    /// Returns a new `Request` instance configured with code to compile
    pub fn new<C>(code: C) -> Self
    where
        C: Into<String>,
    {
        Request { code: code.into() }
    }
}

/// Format operation response
#[derive(Debug, Deserialize)]
pub struct Response {
    /// Indicates if request was successful or not
    pub success: bool,
    /// The formatted code
    pub code: String,
    /// Stdout line ouput
    pub stdout: String,
    /// Stderr line ouput
    pub stderr: String,
}