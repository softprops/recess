extern crate hyper;
extern crate futures;
extern crate recess;
extern crate tokio_core;

use futures::Future;
use recess::{Client, CompileRequest, Error};
use tokio_core::reactor::Core;

fn run() -> Result<(), Error> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let work = client
        .compile(CompileRequest::builder(
            r#"fn main() { println!("{}", 1); }"#,
        ).build()?)
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
