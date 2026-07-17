pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesSyncRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1ReferenceExchangeRatesSyncRequest {
    pub fn builder() -> PostV1ReferenceExchangeRatesSyncRequestBuilder {
        <PostV1ReferenceExchangeRatesSyncRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesSyncRequestBuilder {
    date: Option<String>,
}

impl PostV1ReferenceExchangeRatesSyncRequestBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesSyncRequest`].
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesSyncRequest, BuildError> {
        Ok(PostV1ReferenceExchangeRatesSyncRequest { date: self.date })
    }
}
