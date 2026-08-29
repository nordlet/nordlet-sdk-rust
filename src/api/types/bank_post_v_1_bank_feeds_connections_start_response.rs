pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsStartResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    pub url: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
}

impl PostV1BankFeedsConnectionsStartResponse {
    pub fn builder() -> PostV1BankFeedsConnectionsStartResponseBuilder {
        <PostV1BankFeedsConnectionsStartResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsStartResponseBuilder {
    connection_id: Option<String>,
    reference: Option<String>,
    url: Option<String>,
    expires_at: Option<String>,
}

impl PostV1BankFeedsConnectionsStartResponseBuilder {
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

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsStartResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](PostV1BankFeedsConnectionsStartResponseBuilder::connection_id)
    /// - [`reference`](PostV1BankFeedsConnectionsStartResponseBuilder::reference)
    /// - [`url`](PostV1BankFeedsConnectionsStartResponseBuilder::url)
    /// - [`expires_at`](PostV1BankFeedsConnectionsStartResponseBuilder::expires_at)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsStartResponse, BuildError> {
        Ok(PostV1BankFeedsConnectionsStartResponse {
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
