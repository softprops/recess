//! Compilation interfaces

// https://github.com/colin-kiegel/rust-derive-builder/issues/104
#![allow(unused_mut)]

use std::str::FromStr;

use {AsmFlavor, Channel, CrateType, Mode};

/// Compiler output targets
///
/// The `Default` is `Asm`
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Target {
    #[serde(rename = "asm")]
    Asm,
    #[serde(rename = "llvm-ir")]
    Llvm,
    #[serde(rename = "mir")]
    Mir,
    /// Only available for the Nightly channel
    #[serde(rename = "wasm")]
    Wasm,
}

impl Target {
    pub fn variants() -> &'static [&'static str] {
        &["asm", "llvm-ir", "mir", "wasm"]
    }
}

impl Default for Target {
    fn default() -> Self {
        Target::Asm
    }
}

impl FromStr for Target {
    type Err = &'static str;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "asm" => Ok(Target::Asm),
            "llvm-ir" => Ok(Target::Llvm),
            "mir" => Ok(Target::Mir),
            "wasm" => Ok(Target::Wasm),
            _ => Err("invalid target"),
        }
    }
}

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

// https://github.com/integer32llc/rust-playground/blob/4a49170ea46c4bae244a32b7e460534b56ccf02c/ui/src/main.rs#L523-L541
/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, Builder, PartialEq)]
#[builder(setter(into), default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// The desired compiler output format
    target: Target,
    #[serde(skip_serializing_if = "Option::is_none")]
    assembly_flavor: Option<AsmFlavor>,
    demangle_assembly: DemangleAssembly,
    hide_assembler_directives: HideAssemblerDirectives,
    channel: Channel,
    mode: Mode,
    #[serde(default)]
    edition: String,
    crate_type: CrateType,
    tests: bool,
    #[serde(default)]
    backtrace: bool,
    /// code to compile
    code: String,
}

impl Request {
    /// Return a new Request with default options
    pub fn new<C>(code: C) -> Self
    where
        C: Into<String>,
    {
        Request {
            code: code.into(),
            ..Default::default()
        }
    }
    /// Returns a new `RequestBuilder` instance configured with code to compile
    pub fn builder<C>(code: C) -> RequestBuilder
    where
        C: Into<String>,
    {
        RequestBuilder::default().code(code).clone()
    }
}

// https://github.com/integer32llc/rust-playground/blob/4a49170ea46c4bae244a32b7e460534b56ccf02c/ui/src/main.rs#L543-L549
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
                target: Target::Asm,
                assembly_flavor: None,
                demangle_assembly: DemangleAssembly::Demangle,
                hide_assembler_directives: HideAssemblerDirectives::Hide,
                channel: Channel::Stable,
                mode: Mode::Debug,
                edition: String::new(),
                crate_type: CrateType::Binary,
                tests: false,
                backtrace: false,
                code: String::from("foo"),
            }
        )
    }

}
