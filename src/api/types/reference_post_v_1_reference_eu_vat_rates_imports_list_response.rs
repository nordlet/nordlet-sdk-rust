pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesImportsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceEuVatRatesImportsListResponseRowsItem>,
}

impl PostV1ReferenceEuVatRatesImportsListResponse {
    pub fn builder() -> PostV1ReferenceEuVatRatesImportsListResponseBuilder {
        <PostV1ReferenceEuVatRatesImportsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesImportsListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceEuVatRatesImportsListResponseRowsItem>>,
}

impl PostV1ReferenceEuVatRatesImportsListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1ReferenceEuVatRatesImportsListResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesImportsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceEuVatRatesImportsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesImportsListResponse, BuildError> {
        Ok(PostV1ReferenceEuVatRatesImportsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
