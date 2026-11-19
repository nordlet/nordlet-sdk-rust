pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbHmrcConnectDeclarationsResponse {
    #[serde(rename = "accessTokenExpiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub access_token_expires_at: DateTime<FixedOffset>,
}

impl GbHmrcConnectDeclarationsResponse {
    pub fn builder() -> GbHmrcConnectDeclarationsResponseBuilder {
        <GbHmrcConnectDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbHmrcConnectDeclarationsResponseBuilder {
    access_token_expires_at: Option<DateTime<FixedOffset>>,
}

impl GbHmrcConnectDeclarationsResponseBuilder {
    pub fn access_token_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.access_token_expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbHmrcConnectDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token_expires_at`](GbHmrcConnectDeclarationsResponseBuilder::access_token_expires_at)
    pub fn build(self) -> Result<GbHmrcConnectDeclarationsResponse, BuildError> {
        Ok(GbHmrcConnectDeclarationsResponse {
            access_token_expires_at: self
                .access_token_expires_at
                .ok_or_else(|| BuildError::missing_field("access_token_expires_at"))?,
        })
    }
}
