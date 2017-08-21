use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::StatusCode;
use serde_json::error::Error as SerdeError;
use hyper::error::UriError;

error_chain! {
  errors {
      Fault {
          code: StatusCode,
          error: String,
      }
  }
  foreign_links {
      Codec(SerdeError);
      Http(HttpError);
      IO(IoError);
      Uri(UriError);
  }
}
