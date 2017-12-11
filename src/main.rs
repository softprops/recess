extern crate futures;
extern crate recess;
extern crate tokio_core;

use futures::Future;
use recess::*;
use tokio_core::reactor::Core;

fn run() -> Result<()> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let work = client
        .lint(LintRequest::new(
            r#"fn main() { println!("{}", if true { true } else { false }); }"#,
        ))
        .and_then(|result| {
            for line in result.stdout.lines() {
                println!("{}", line);
            }
            for line in result.stderr.lines() {
                println!("{}", line);
            }
            Ok(())
        });

    core.run(work).map_err(recess::Error::from).map(|_| ())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err)
    }
}
