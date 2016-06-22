extern crate hyper;

use hyper::Client;
use std::io;

fn main() {
    let cli = Client::new();
    let payload = r#"{"code":"fn main() {\n  println!(\"{}\", \"let's play\")\n}\n","version":"stable","optimize":"0","test":false,"separate_output":true,"color":true,"backtrace":"1"}"#;
    match cli.post("https://play.rust-lang.org/evaluate.json").body(payload).send() {
        Ok(mut resp) => { io::copy(&mut resp, &mut io::stdout()).unwrap(); },
        Err(_) => println!("go back to class")
    }
}
