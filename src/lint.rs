//! Linting interfaces

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Request {
    code: String,
}

impl Request {
    /// Returns a new `RequestBuilder` instance configured with code to compile
    pub fn new<C>(code: C) -> Self
    where
        C: Into<String>,
    {
        Request { code: code.into() }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_builder_defaults() {
        assert_eq!(
            Request::builder("foo").build().unwrap(),
            Request {
                target: CompileOutput::Asm,
                assembly_flavor: None,
                channel: Channel::Stable,
                mode: Mode::Debug,
                crate_type: CrateType::Binary,
                tests: false,
                code: String::from("foo"),
            }
        )
    }

}
