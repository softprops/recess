//use hyper::error::UriError;
use hyper::Error as HttpError;
use hyper::StatusCode;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Codec(#[cause] SerdeError),
    #[fail(display = "{}", _0)]
    Http(#[cause] HttpError),
    #[fail(display = "{}", _0)]
    Io(#[cause] IoError),
    #[fail(display = "{}", _0)]
    Fault(StatusCode), //#[fail(display = "{}", _0)]
                       //Uri(#[cause] UriError)
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error::Http(err)
    }
}

impl From<StatusCode> for Error {
    fn from(err: StatusCode) -> Self {
        Error::Fault(err)
    }
}

// error_chain! {
//   errors {
//       Fault {
//           code: StatusCode,
//           error: String,
//       } {
//             display("{}: '{}'", code, error)
//             description(error.as_str())
//           }
//   }
//   foreign_links {
//       Codec(SerdeError);
//       Http(HttpError);
//       IO(IoError);
//       Uri(UriError);
//   }
// }
