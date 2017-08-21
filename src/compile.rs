use super::{AsmFlavor, CrateType, Channel, CompileOutput, Mode};

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
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}