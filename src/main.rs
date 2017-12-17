extern crate futures;
extern crate recess;
extern crate tokio_core;
extern crate clap;

use std::io::stdin;

use clap::{App, Arg, ArgMatches, SubCommand};
use futures::Future;
use tokio_core::reactor::Core;

use recess::*;

fn run(matches: ArgMatches<'static>) -> Result<()> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    match matches.subcommand_name() {
        Some("compile") => {
            let sub_matches = matches.subcommand_matches("compile").unwrap();
            let src = match sub_matches.value_of("src").unwrap() {
                "-" => {
                    let mut buffer = String::new();
                    stdin().read_line(&mut buffer).expect(
                        "failed to read line",
                    );
                    buffer
                }
                src => src.to_string(),
            };

            let mut options = CompileRequest::builder(src);

            for value in sub_matches.value_of("channel").and_then(|s| {
                s.parse::<Channel>().ok()
            })
            {
                options.channel(value);
            }

            for value in sub_matches.value_of("target").and_then(|s| {
                s.parse::<CompileOutput>().ok()
            })
            {
                options.target(value);
            }

            for value in sub_matches.value_of("crate_type").and_then(|s| {
                s.parse::<CrateType>().ok()
            })
            {
                options.crate_type(value);
            }

            let f = client.compile(options.build()?).and_then(|result| {
                for line in result.code.lines() {
                    println!("{}", line);
                }
                for line in result.stdout.lines() {
                    println!("{}", line);
                }
                for line in result.stderr.lines() {
                    println!("{}", line);
                }
                Ok(())
            });

            core.run(f).map_err(recess::Error::from).map(|_| ())
        }
        Some("exec") => Ok(()),
        Some("fmt") => Ok(()),
        Some("lint") => Ok(()),
        _ => Ok(()),
    }
}

fn main() {
    if let Err(err) = run(cli().get_matches()) {
        eprintln!("{}", err)
    }
}

fn cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .about("rust playground cli")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("compile")
                .about("compiles rust source code")
                .arg(
                    Arg::with_name("target")
                        .long("target")
                        .help("compile output target")
                        .possible_values(&["asm","llvm-ir", "mir","wasm"])
                        .value_name("target")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("channel")
                        .long("channel")
                        .help("rustc channel")
                        .possible_values(&["stable", "beta", "nightly"])
                        .value_name("channel")
                        .takes_value(true)
                ).arg(
                    Arg::with_name("crate_type")
                        .long("crate_type")
                        .help("crate type")
                        .possible_values(&["lib", "bin"])
                        .value_name("crate_type")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("src")
                        .help("code to compile. code is read from std in if not provided")
                        .takes_value(true)
                        //.possible_values(&["fn main() { ... }", "@path/to/file.rs", "-"])
                        .required(true)
                        .value_name("src"),
                ),
        )
        .subcommand(
            SubCommand::with_name("exec")
                .about("executes rust source code")
                .arg(
                    Arg::with_name("src")
                        .help("code to compile. code is read from std in if not provided")
                        .takes_value(true)
                        //.possible_values(&["fn main() { ... }", "@path/to/file.rs", "-"])
                        .required(true)
                        .value_name("src"),
                ),
        )
        .subcommand(
            SubCommand::with_name("fmt")
                .about("formats rust source code")
                .arg(
                    Arg::with_name("src")
                        .help("code to compile. code is read from std in if not provided")
                        .takes_value(true)
                        //.possible_values(&["fn main() { ... }", "@path/to/file.rs", "-"])
                        .required(true)
                        .value_name("src"),
                ),
        )
        .subcommand(
            SubCommand::with_name("lint")
                .about("lints rust source code")
                .arg(
                    Arg::with_name("src")
                        .help("code to compile. code is read from std in if not provided")
                        .takes_value(true)
                        //.possible_values(&["fn main() { ... }", "@path/to/file.rs", "-"])
                        .required(true)
                        .value_name("src"),
                ),
        )
}
