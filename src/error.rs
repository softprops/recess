use hyper::Error as HttpError;
use hyper::StatusCode;
use hyper::error::UriError;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;

error_chain! {
  errors {
      Fault {
          code: StatusCode,
          error: String,
      } {
            display("{}: '{}'", code, error)
            description(error.as_str())
          }
  }
  foreign_links {
      Codec(SerdeError);
      Http(HttpError);
      IO(IoError);
      Uri(UriError);
  }
}
