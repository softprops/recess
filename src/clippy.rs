//https://github.com/integer32llc/rust-playground/blob/4a49170ea46c4bae244a32b7e460534b56ccf02c/ui/src/main.rs#L585-L588
/// A clippy linting request
#[derive(Debug, Serialize, Default, Builder, PartialEq)]
#[builder(setter(into), default)]
pub struct Request {
  code: String,
}

// https://github.com/integer32llc/rust-playground/blob/4a49170ea46c4bae244a32b7e460534b56ccf02c/ui/src/main.rs#L590-L595
/// A clippy linting response
#[derive(Debug, Deserialize)]
pub struct Response {
  success: bool,
  stdout: String,
  stderr: String,
}
