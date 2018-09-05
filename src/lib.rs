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
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "tls")]
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use std::str::FromStr;

use futures::Future as StdFuture;
use futures::Stream;
use hyper::client::connect::Connect;
use hyper::client::HttpConnector;
use hyper::{Body, Method, Request, Uri};
//use hyper::header::ContentType;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub mod clippy;
pub mod compile;
pub mod execute;
pub mod format;
pub mod lint;

pub use clippy::Request as ClippyRequest;
pub use compile::Request as CompileRequest;
pub use execute::Request as ExecuteRequest;
pub use format::Request as FormatRequest;
pub use lint::Request as LintRequest;

mod error;
pub use error::*;

#[derive(Debug, Deserialize, PartialEq)]
struct ClientError {
    pub error: String,
}

/// A type alias for futures that may return recess::Error's
pub type Future<T> = Box<StdFuture<Item = T, Error = Error> + Send>;

/// Type of crate
///
/// The `Default` is `Binary`
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum CrateType {
    /// Rust binary
    #[serde(rename = "bin")]
    Binary,
    /// Rust library
    #[serde(rename = "lib")]
    Library,
}

impl CrateType {
    pub fn variants() -> &'static [&'static str] {
        &["bin", "lib"]
    }
}

impl Default for CrateType {
    fn default() -> Self {
        CrateType::Binary
    }
}

impl FromStr for CrateType {
    type Err = &'static str;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "bin" => Ok(CrateType::Binary),
            "lib" => Ok(CrateType::Library),
            _ => Err("invalid crate_type"),
        }
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

impl FromStr for Mode {
    type Err = &'static str;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Mode::Debug),
            "release" => Ok(Mode::Release),
            _ => Err("invalid mode"),
        }
    }
}

/// Release train options.
///
/// The `Default` is `Stable`
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    /// stable release
    Stable,
    /// beta release
    Beta,
    /// nightly release
    Nightly,
}

impl Channel {
    pub fn variants() -> &'static [&'static str] {
        &["stable", "beta", "nightly"]
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Stable
    }
}

impl FromStr for Channel {
    type Err = &'static str;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "nightly" => Ok(Channel::Nightly),
            _ => Err("invalid channel"),
        }
    }
}

/// Assembly flavor.
///
/// The `Default` is `Att`
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AsmFlavor {
    /// AT&T assembly
    Att,
    /// Intell assembly
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

/// Rust playground client
pub struct Client<C = HttpsConnector<HttpConnector>>
where
    C: Connect + Clone + 'static,
{
    host: Uri,
    http: hyper::Client<C>,
}

/// An implementation of Client for HttpsConnectors
#[cfg(feature = "tls")]
impl Client<HttpsConnector<HttpConnector>> {
    /// Creates a new instance of a `Client` using a `hyper::Client`
    /// preconfigured for tls.
    ///
    /// For client customization use `Client::custom` instead
    pub fn new() -> Self {
        let connector = HttpsConnector::new(4).unwrap();
        let hyper = hyper::Client::builder().keep_alive(true).build(connector);
        Client::custom("https://play.rust-lang.org".parse().unwrap(), hyper)
    }
}

impl<C> Client<C>
where
    C: Clone + Connect + 'static,
{
    /// Creates a new playground
    pub fn custom(host: Uri, http: hyper::Client<C>) -> Self {
        Self { host, http }
    }

    /// Executes rustlang code
    pub fn execute(&self, req: ExecuteRequest) -> Future<execute::Response> {
        self.request::<execute::Request, execute::Response>(
            "https://play.rust-lang.org/execute",
            req,
        )
    }

    /// Compiles rustlang code
    pub fn compile(&self, req: CompileRequest) -> Future<compile::Response> {
        self.request::<CompileRequest, compile::Response>(
            "https://play.rust-lang.org/compile",
            req,
        )
    }

    /// Formats rustlang code
    pub fn format(&self, req: format::Request) -> Future<format::Response> {
        self.request::<format::Request, format::Response>(
            "https://play.rust-lang.org/format",
            req,
        )
    }

    /// Lint rustlang code
    pub fn lint(&self, req: lint::Request) -> Future<lint::Response> {
        self.request::<lint::Request, lint::Response>(
            "https://play.rust-lang.org/clippy",
            req,
        )
    }

    fn request<I, O>(&self, url: &str, input: I) -> Future<O>
    where
        I: Serialize,
        O: DeserializeOwned + 'static + Send,
    {
        let mut builder = Request::builder();
        builder.method(Method::POST);
        builder.uri(url);
        builder.header("Content-Type", "application/json");
        //req.headers_mut().set(ContentType::json());
        let req = builder
            .body(Body::from(serde_json::to_vec(&input).unwrap()))
            .unwrap();
        Box::new(self.http.request(req).map_err(Error::from).and_then(
            |response| {
                let status = response.status();
                let body = response.into_body().concat2().map_err(Error::from);
                body.and_then(move |body| {
                    if status.is_success() {
                        serde_json::from_slice::<O>(&body)
                            .map_err(|err| Error::Codec(err).into())
                    } else {
                        match serde_json::from_slice::<ClientError>(&body) {
                            Ok(_) => Err(Error::Fault(status).into()),
                            Err(error) => Err(Error::Codec(error).into()),
                        }
                    }
                })
            },
        ))
    }
}
