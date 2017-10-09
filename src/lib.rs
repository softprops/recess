//! An interface for interacting with the Rust [playground](https://play.rust-lang.org/)
//!
//! # examples
//!
//! ```no_run
//! // recess interfaces
//! extern crate recess;
//! // tokio async io
//! extern crate tokio_core;
//! // futures combinators
//! extern crate futures;
//!
//! use recess::{Client, CompileRequest};
//! use futures::Future;
//! use tokio_core::reactor::Core;
//!
//! fn main() {
//!   let mut core = Core::new().unwrap();
//!   let client = Client::new(
//!      &core.handle(),
//!   );
//!
//!   let work = client.compile(CompileRequest::builder(
//!              r#"fn main() { println!("{}", 1); }"#
//!            )
//!           .build().unwrap())
//!        .and_then(|result| {
//!            println!("{}", result.stdout);
//!            println!("{}", result.stderr);
//!            Ok(())
//!        });
//!
//!   println!("{:#?}", core.run(work))
//! }
//! ```
//!
//! # Cargo features
//!
//! This crate has one Cargo feature, `tls`, which adds HTTPS support via the `Client::new`
//! constructor. This feature is enabled by default.
#![warn(missing_docs)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;
#[cfg(feature = "tls")]
extern crate hyper_tls;

use futures::Future as StdFuture;
use futures::Stream;
use hyper::{Method, Request};
use hyper::client::{Connect, HttpConnector};
use hyper::header::ContentType;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Handle;

#[derive(Debug, Deserialize, PartialEq)]
struct ClientError {
    pub error: String,
}

mod error;
pub use error::*;

/// A type alias for futures that may return recess::Error's
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum CrateType {
    /// Rust binary
    #[serde(rename = "bin")]
    Binary,
    /// Rust library
    #[serde(rename = "lib")]
    Library,
}

impl Default for CrateType {
    fn default() -> Self {
        CrateType::Binary
    }
}

/// Rustc compilation mode.
///
/// The `Default` is `Debug`
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Mode {
    /// Debug compilation mode
    #[serde(rename = "debug")]
    Debug,
    /// Release compilation mode
    #[serde(rename = "release")]
    Release,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Debug
    }
}

/// Release train options.
///
/// The `Default` is `Stable`
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Channel {
    /// stable release
    #[serde(rename = "stable")]
    Stable,
    /// beta release
    #[serde(rename = "beta")]
    Beta,
    /// nightly release
    #[serde(rename = "nightly")]
    Nightly,
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Stable
    }
}


/// Assembly flavor.
///
/// The `Default` is `Att`
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AsmFlavor {
    /// AT&T
    #[serde(rename = "att")]
    Att,
    /// Intell
    #[serde(rename = "intel")]
    Intel,
}

impl Default for AsmFlavor {
    fn default() -> Self {
        AsmFlavor::Att
    }
}

/// Rustc backtrace options
#[derive(Debug, Serialize)]
pub enum Backtrace {
    /// No backtraces
    #[serde(rename = "0")]
    Never,
    /// Always return backtraces
    #[serde(rename = "1")]
    Always,
    /// Detect when to return backtraces
    #[serde(rename = "2")]
    Auto,
}

impl Default for Backtrace {
    fn default() -> Self {
        Backtrace::Auto
    }
}

/// Optimization levels for rustc
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

/// Rust playground client
pub struct Client<C>
where
    C: Connect + Clone,
{
    http: hyper::Client<C>,
}

/// An implementation of Client for HttpsConnectors
#[cfg(feature = "tls")]
impl Client<HttpsConnector<HttpConnector>> {
    /// Creates a new instance of a `Client` using a `hyper::Client`
    /// preconfigured for tls.
    ///
    /// For client customization use `Client::custom` instead
    pub fn new(handle: &Handle) -> Self {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let hyper = hyper::Client::configure()
            .connector(connector)
            .keep_alive(true)
            .build(handle);
        Client::custom(hyper)
    }
}


impl<C> Client<C>
where
    C: Clone + Connect,
{
    /// Creates a new playground
    pub fn custom(http: hyper::Client<C>) -> Self {
        Self { http }
    }

    /// Executes rustlang code
    pub fn execute(&self, req: execute::Request) -> Future<execute::Response> {
        self.request::<execute::Request, execute::Response>(
            "https://play.rust-lang.org/execute",
            req,
        )
    }

    /// Compiles rustlang code
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
                    serde_json::from_slice::<O>(&body).map_err(|err| {
                        ErrorKind::Codec(err).into()
                    })
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