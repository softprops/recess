//! Linting interfaces

// https://github.com/colin-kiegel/rust-derive-builder/issues/104
#![allow(unused_mut)]

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

/// Lint operation response
#[derive(Debug, Deserialize)]
pub struct Response {
    /// Indicates if request was successful or not
    pub success: bool,
    /// Stdout line ouput
    pub stdout: String,
    /// Stderr line output
    pub stderr: String,
}