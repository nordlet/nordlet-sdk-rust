pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsStartBankResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    pub url: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl FeedsConnectionsStartBankResponse {
    pub fn builder() -> FeedsConnectionsStartBankResponseBuilder {
        <FeedsConnectionsStartBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsStartBankResponseBuilder {
    connection_id: Option<String>,
    reference: Option<String>,
    url: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl FeedsConnectionsStartBankResponseBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsStartBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](FeedsConnectionsStartBankResponseBuilder::connection_id)
    /// - [`reference`](FeedsConnectionsStartBankResponseBuilder::reference)
    /// - [`url`](FeedsConnectionsStartBankResponseBuilder::url)
    /// - [`expires_at`](FeedsConnectionsStartBankResponseBuilder::expires_at)
    pub fn build(self) -> Result<FeedsConnectionsStartBankResponse, BuildError> {
        Ok(FeedsConnectionsStartBankResponse {
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
