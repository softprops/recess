//! Compilation interfaces

use {AsmFlavor, Channel, CompileOutput, CrateType, Mode};

/// Demangling options
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DemangleAssembly {
    Demangle,
    Mangle,
}

impl Default for DemangleAssembly {
    fn default() -> Self {
        DemangleAssembly::Demangle
    }
}

/// Assembler visibility options
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HideAssemblerDirectives {
    Hide,
    Show,
}

impl Default for HideAssemblerDirectives {
    fn default() -> Self {
        HideAssemblerDirectives::Hide
    }
}

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, Builder, PartialEq)]
#[builder(setter(into), default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// The deserired compiler output format
    target: CompileOutput,
    #[serde(skip_serializing_if = "Option::is_none")]
    assembly_flavor: Option<AsmFlavor>,
    demangle_assembly: DemangleAssembly,
    hide_assembler_directives: HideAssemblerDirectives,
    channel: Channel,
    mode: Mode,
    crate_type: CrateType,
    tests: bool,
    /// code to compile
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

/// Compile operation response
#[derive(Debug, Deserialize)]
pub struct Response {
    /// Indicates if request was successful or not
    pub success: bool,
    /// Compiled code
    pub code: String,
    /// Stdout line ouput
    pub stdout: String,
    /// Stderr line ouput
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
                demangle_assembly: DemangleAssembly::Demangle,
                hide_assembler_directives: HideAssemblerDirectives::Hide,
                channel: Channel::Stable,
                mode: Mode::Debug,
                crate_type: CrateType::Binary,
                tests: false,
                code: String::from("foo"),
            }
        )
    }

}
