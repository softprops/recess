use super::{Channel, CrateType, Mode};

/// Parameters for compiling rustlang code
#[derive(Debug, Serialize, Default, Builder, PartialEq)]
#[builder(setter(into), default)]
pub struct Request {
    pub channel: Channel,
    pub mode: Mode,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub tests: bool,
    pub code: String,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}
