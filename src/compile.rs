//! Compilation interfaces

use super::{AsmFlavor, Channel, CompileOutput, CrateType, Mode};

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, Builder, PartialEq)]
#[builder(setter(into), default)]
pub struct Request {
    target: CompileOutput,
    #[serde(rename = "assemblyFlavor", skip_serializing_if = "Option::is_none")]
    assembly_flavor: Option<AsmFlavor>,
    channel: Channel,
    mode: Mode,
    #[serde(rename = "crateType")]
    crate_type: CrateType,
    tests: bool,
    code: String,
}

impl Request {
    /// Returns a new `RequestBuilder` instance configured with code to compile
    pub fn builder<C>(code: C) -> RequestBuilder
    where
        C: Into<String>,
    {
        RequestBuilder::default().code(code).clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: bool,
    pub code: String,
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
