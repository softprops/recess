extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate recess;
extern crate tokio_core;

use futures::Future;
use hyper_tls::HttpsConnector;
use recess::{Client, Error, CompileRequest};
use tokio_core::reactor::Core;

fn run() -> Result<(), recess::Error> {
    let mut core = Core::new()?;
    let client = Client::new(
        hyper::Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle()),
    );
    let work = client
        .compile(
            CompileRequest::builder()
                .code(r#"fn main() { println!("{}", 1); }"#)
                .build()
                .unwrap(),
        )
        .and_then(|result| {
            println!("{}", result.stdout);
            println!("{}", result.stderr);
            Ok(())
        });

    core.run(work).map_err(recess::Error::from).map(|_| ())
}

fn main() {
    run().unwrap();
}
