pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbHmrcConnectDeclarationsRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub state: String,
}

impl GbHmrcConnectDeclarationsRequest {
    pub fn builder() -> GbHmrcConnectDeclarationsRequestBuilder {
        <GbHmrcConnectDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbHmrcConnectDeclarationsRequestBuilder {
    code: Option<String>,
    state: Option<String>,
}

impl GbHmrcConnectDeclarationsRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbHmrcConnectDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GbHmrcConnectDeclarationsRequestBuilder::code)
    /// - [`state`](GbHmrcConnectDeclarationsRequestBuilder::state)
    pub fn build(self) -> Result<GbHmrcConnectDeclarationsRequest, BuildError> {
        Ok(GbHmrcConnectDeclarationsRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
