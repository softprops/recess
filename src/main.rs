extern crate clap;
extern crate futures;
extern crate recess;
extern crate tokio;
#[macro_use]
extern crate structopt;

use std::io::stdin;

use futures::Future;
use recess::compile::Target;
use recess::{
    Channel, Client, CompileRequest, CrateType, ExecuteRequest, FormatRequest,
};
use structopt::StructOpt;
use tokio::runtime::Runtime;

/// CLI options
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "recess", about = "Rust playground cli")]
enum Options {
    #[structopt(
        name = "execute", alias = "exec", about = "Execute source code"
    )]
    Execute {
        #[structopt(short = "s", long = "src")]
        code: String,
        #[structopt(
            short = "c",
            long = "channel",
            raw(possible_values = "&Channel::variants()")
        )]
        channel: Option<Channel>,
        #[structopt(
            long = "crate_type", raw(possible_values = "&CrateType::variants()")
        )]
        crate_type: Option<CrateType>,
    },
    #[structopt(name = "compile", about = "Compile source code")]
    Compile {
        #[structopt(short = "s", long = "src")]
        code: String,
        #[structopt(
            short = "t",
            long = "target",
            raw(possible_values = "&Target::variants()")
        )]
        target: Option<Target>,
        #[structopt(
            short = "c",
            long = "channel",
            raw(possible_values = "&Channel::variants()")
        )]
        channel: Option<Channel>,
        #[structopt(
            long = "crate_type", raw(possible_values = "&CrateType::variants()")
        )]
        crate_type: Option<CrateType>,
    },
    #[structopt(name = "format", alias = "fmt", about = "Format source code")]
    Format {
        #[structopt(short = "s", long = "src")]
        code: String,
    },
}

fn src(code: String) -> String {
    if code != "-" {
        return code;
    }
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("failed to read line");
    buffer
}

fn main() {
    let mut runtime = Runtime::new().expect("failed to initialize runtime");
    let result = match Options::from_args() {
        Options::Execute {
            code,
            channel,
            crate_type,
        } => {
            let mut options = ExecuteRequest::builder(src(code));

            for c in channel {
                options.channel(c);
            }
            for t in crate_type {
                options.crate_type(t);
            }

            let response = Client::new()
                .execute(options.build().unwrap())
                .and_then(|result| {
                    for line in result.stdout.lines() {
                        println!("{}", line);
                    }
                    for line in result.stderr.lines() {
                        eprintln!("{}", line);
                    }
                    Ok(())
                });

            runtime
                .block_on(response)
                .map_err(recess::Error::from)
                .map(|_| ())
        }
        Options::Compile {
            code,
            target,
            channel,
            crate_type,
        } => {
            let mut options = CompileRequest::builder(src(code));
            for t in target {
                options.target(t);
            }
            for c in channel {
                options.channel(c);
            }
            for t in crate_type {
                options.crate_type(t);
            }

            let response = Client::new()
                .compile(options.build().unwrap())
                .and_then(|result| {
                    for line in result.code.lines() {
                        println!("{}", line);
                    }
                    for line in result.stdout.lines() {
                        println!("{}", line);
                    }
                    for line in result.stderr.lines() {
                        eprintln!("{}", line);
                    }
                    Ok(())
                });

            runtime
                .block_on(response)
                .map_err(recess::Error::from)
                .map(|_| ())
        }
        Options::Format { code } => {
            let response = Client::new()
                .format(FormatRequest::new(src(code)))
                .and_then(|result| {
                    for line in result.code.lines() {
                        println!("{}", line);
                    }
                    for line in result.stdout.lines() {
                        println!("{}", line);
                    }
                    for line in result.stderr.lines() {
                        eprintln!("{}", line);
                    }
                    Ok(())
                });
            runtime
                .block_on(response)
                .map_err(recess::Error::from)
                .map(|_| ())
        }
    };

    if let Err(err) = result {
        eprintln!("{}", err)
    }
}
