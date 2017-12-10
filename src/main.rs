extern crate futures;
extern crate recess;
extern crate tokio_core;

use futures::Future;
use recess::{Channel, Client, CompileOutput, CompileRequest, Error,
             ExecuteRequest, FormatRequest, LintRequest};
use tokio_core::reactor::Core;

fn run() -> Result<(), Error> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let work = client
        .lint(LintRequest::new(
            r#"fn main() { println!("{}", if true { true } else { false }); }"#,
        ))
        .and_then(|result| {
            for line in result.stderr.lines() {
                println!("{}", line);
            }
            Ok(())
        });

    core.run(work).map_err(recess::Error::from).map(|_| ())
}

fn main() {
    run().unwrap();
}
