#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use futures::Future as StdFuture;
use futures::Stream;
use hyper::{Method, Request};
use hyper::client::Connect;
use hyper::header::ContentType;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize, PartialEq)]
struct ClientError {
    pub error: String,
}

mod error;
pub use error::*;

/// A type alias for futures that may return travis::Error's
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum CrateType {
    #[serde(rename = "bin")]
    Binary,
    #[serde(rename = "lib")]
    Library,
}

impl Default for CrateType {
    fn default() -> Self {
        CrateType::Binary
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Mode {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "release")]
    Release,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Debug
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Channel {
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "nightly")]
    Nightly,
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Stable
    }
}


#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AsmFlavor {
    #[serde(rename = "att")]
    Att,
    #[serde(rename = "intel")]
    Intel,
}

impl Default for AsmFlavor {
    fn default() -> Self {
        AsmFlavor::Att
    }
}

#[derive(Debug, Serialize)]
pub enum Backtrace {
    #[serde(rename = "0")]
    Never,
    #[serde(rename = "1")]
    Always,
    #[serde(rename = "2")]
    Auto,
}

impl Default for Backtrace {
    fn default() -> Self {
        Backtrace::Auto
    }
}

#[derive(Debug, Serialize)]
pub enum OptLevel {
    #[serde(rename = "0")]
    O0,
    #[serde(rename = "1")]
    O1,
    #[serde(rename = "2")]
    O2,
    #[serde(rename = "3")]
    O3,
}

/// Compiler output formats
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum CompileOutput {
    #[serde(rename = "asm")]
    Asm,
    #[serde(rename = "llvm-ir")]
    Llvm,
    #[serde(rename = "mir")]
    Mir,
}

impl Default for CompileOutput {
    fn default() -> Self {
        CompileOutput::Asm
    }
}

pub mod compile;
// re-export CompileRequest
pub use compile::Request as CompileRequest;

pub mod execute;
// re-export ExportRequest
pub use execute::Request as ExecuteRequest;

/// rust playground client
pub struct Client<C>
where
    C: Connect + Clone,
{
    http: hyper::Client<C>,
}

impl<C> Client<C>
where
    C: Clone + Connect,
{
    /// create a new playground
    pub fn new(http: hyper::Client<C>) -> Self {
        Self { http }
    }

    /// execute rustlang code
    pub fn execute(&self, req: execute::Request) -> Future<execute::Response> {
        self.request::<execute::Request, execute::Response>(
            "https://play.rust-lang.org/execute",
            req,
        )
    }

    /// compile rustlang code
    pub fn compile(&self, req: compile::Request) -> Future<compile::Response> {
        self.request::<compile::Request, compile::Response>(
            "https://play.rust-lang.org/compile",
            req,
        )
    }

    fn request<I, O>(&self, url: &str, input: I) -> Future<O>
    where
        I: Serialize,
        O: DeserializeOwned + 'static,
    {
        let mut req = Request::new(Method::Post, url.parse().unwrap());
        req.headers_mut().set(ContentType::json());
        req.set_body(serde_json::to_vec(&input).unwrap());
        Box::new(self.http.request(req).map_err(Error::from).and_then(
            |response| {
                let status = response.status();
                let body = response.body().concat2().map_err(Error::from);
                body.and_then(move |body| if status.is_success() {
                    serde_json::from_slice::<O>(&body).map_err(|err| ErrorKind::Codec(err).into())
                } else {
                    match serde_json::from_slice::<ClientError>(&body) {
                        Ok(error) => Err(
                            ErrorKind::Fault {
                                code: status,
                                error: error.error,
                            }.into(),
                        ),
                        Err(error) => Err(ErrorKind::Codec(error).into()),
                    }
                })
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_builder_defaults() {
        assert_eq!(
            CompileBuilder::default().build().unwrap(),
            Compile {
                target: CompileOutput::Asm,
                assembly_flavor: None,
                channel: Channel::Stable,
                mode: Mode::Debug,
                crate_type: CrateType::Library,
                tests: false,
                code: String::new(),
            }
        )
    }

    #[test]
    fn execute_builder_defaults() {
        assert_eq!(
            ExecuteBuilder::default().build().unwrap(),
            Execute {
                channel: Channel::Stable,
                mode: Mode::Debug,
                crate_type: CrateType::Library,
                tests: false,
                code: String::new(),
            }
        )
    }
}
