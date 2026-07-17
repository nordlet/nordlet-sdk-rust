pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceEuVatRatesListResponseRowsItem>,
}

impl PostV1ReferenceEuVatRatesListResponse {
    pub fn builder() -> PostV1ReferenceEuVatRatesListResponseBuilder {
        <PostV1ReferenceEuVatRatesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceEuVatRatesListResponseRowsItem>>,
}

impl PostV1ReferenceEuVatRatesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceEuVatRatesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceEuVatRatesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesListResponse, BuildError> {
        Ok(PostV1ReferenceEuVatRatesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
