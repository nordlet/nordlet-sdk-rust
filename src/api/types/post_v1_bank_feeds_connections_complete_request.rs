pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsCompleteRequest {
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    pub code: String,
}

impl PostV1BankFeedsConnectionsCompleteRequest {
    pub fn builder() -> PostV1BankFeedsConnectionsCompleteRequestBuilder {
        <PostV1BankFeedsConnectionsCompleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsCompleteRequestBuilder {
    reference: Option<String>,
    code: Option<String>,
}

impl PostV1BankFeedsConnectionsCompleteRequestBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsCompleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](PostV1BankFeedsConnectionsCompleteRequestBuilder::reference)
    /// - [`code`](PostV1BankFeedsConnectionsCompleteRequestBuilder::code)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsCompleteRequest, BuildError> {
        Ok(PostV1BankFeedsConnectionsCompleteRequest {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
