pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesImportsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl PostV1ReferenceEuVatRatesImportsListRequest {
    pub fn builder() -> PostV1ReferenceEuVatRatesImportsListRequestBuilder {
        <PostV1ReferenceEuVatRatesImportsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesImportsListRequestBuilder {
    limit: Option<i64>,
}

impl PostV1ReferenceEuVatRatesImportsListRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesImportsListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesImportsListRequest, BuildError> {
        Ok(PostV1ReferenceEuVatRatesImportsListRequest { limit: self.limit })
    }
}
