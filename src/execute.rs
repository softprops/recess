//! Execution interfaces

// https://github.com/colin-kiegel/rust-derive-builder/issues/104
#![allow(unused_mut)]

use {Channel, CrateType, Mode};

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, Builder, PartialEq)]
#[builder(setter(into), default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// rust release channel
    channel: Channel,
    /// compilation mode
    mode: Mode,
    /// crate type
    crate_type: CrateType,
    /// contains tests
    tests: bool,
    /// source code
    code: String,
}

impl Request {
    /// Returns a new `RequestBuilder` instance configured with code to execute
    pub fn builder<C>(code: C) -> RequestBuilder
    where
        C: Into<String>,
    {
        RequestBuilder::default().code(code).clone()
    }
}

/// Execute operation response
#[derive(Debug, Deserialize)]
pub struct Response {
    /// Indicates if request was successful or not
    pub success: bool,
    /// Stdout line ouput
    pub stdout: String,
    /// Stderr line ouput
    pub stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_builder_defaults() {
        assert_eq!(
            Request::builder("foo").build().unwrap(),
            Request {
                channel: Channel::Stable,
                mode: Mode::Debug,
                crate_type: CrateType::Binary,
                tests: false,
                code: String::from("foo"),
            }
        )
    }
}
