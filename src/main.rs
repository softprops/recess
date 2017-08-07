extern crate futures;
#[macro_use]
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_timer;
extern crate url;
#[macro_use]
extern crate error_chain;
extern crate hyper_tls;
#[macro_use]
extern crate derive_builder;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use hyper_tls::HttpsConnector;
use hyper::{Client, Method, Request};
use hyper::client::Connect;
use hyper::header::ContentType;
use tokio_core::reactor::Core;
use futures::Future as StdFuture;
use futures::Stream;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClientError {
    pub error: String,
}

mod errors {
    use std::io::Error as IoError;
    use hyper::Error as HttpError;
    use hyper::StatusCode;
    use serde_json::error::Error as SerdeError;
    use hyper::error::UriError;
    use super::ClientError;
    error_chain! {
        errors {
            Fault {
                code: StatusCode,
                error: ClientError,
            }
        }
        foreign_links {
            Codec(SerdeError);
            Http(HttpError);
            IO(IoError);
            Uri(UriError);
        }
    }
}
use errors::*;

/// A type alias for futures that may return travis::Error's
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Default, Builder)]
#[builder(setter(into), default)]
pub struct Execute {
    pub channel: Channel,
    pub mode: Mode,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub tests: bool,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Default, Builder)]
#[builder(setter(into), default)]
pub struct Compile {
    target: CompileOutput,
    #[serde(rename = "assemblyFlavor", skip_serializing_if = "Option::is_none")]
    assembly_flavor: Option<AsmFlavor>,
    channel: Channel,
    mode: Mode,
    #[serde(rename = "crateType")]
    crate_type: CrateType,
    tests: bool,
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct CompileResult {
    success: bool,
    code: String,
    stdout: String,
    stderr: String,
}

pub struct Playground<C>
where
    C: Connect + Clone,
{
    http: Client<C>,
}

impl<C> Playground<C>
where
    C: Clone + Connect,
{
    pub fn new(http: Client<C>) -> Self {
        Playground { http: http }
    }
    pub fn execute(&self, req: Execute) -> Future<ExecuteResult> {
        self.request::<Execute, ExecuteResult>("https://play.rust-lang.org/execute", req)
    }

    pub fn compile(&self, req: Compile) -> Future<CompileResult> {
        self.request::<Compile, CompileResult>("https://play.rust-lang.org/compile", req)
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
                    match serde_json::from_slice::<O>(&body) {
                        Ok(data) => Ok(data),
                        Err(error) => Err(ErrorKind::Codec(error).into()),
                    }
                } else {
                    match serde_json::from_slice::<ClientError>(&body) {
                        Ok(error) => Err(
                            ErrorKind::Fault {
                                code: status,
                                error: error,
                            }.into(),
                        ),
                        Err(error) => Err(ErrorKind::Codec(error).into()),
                    }
                })
            },
        ))
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let playground = Playground::new(
        Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle()),
    );
    let work = playground
        .execute(
            ExecuteBuilder::default()
                .code(r#"fn main() { println!("{}", 1); }"#)
                .build()
                .unwrap(),
        )
        .and_then(|result| {
            println!("{}", result.stdout);
            println!("{}", result.stderr);
            Ok(())
        });

    core.run(work).unwrap();
}
