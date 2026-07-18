pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesListResponse {
    #[serde(default)]
    pub notice: String,
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
    notice: Option<String>,
    rows: Option<Vec<PostV1ReferenceEuVatRatesListResponseRowsItem>>,
}

impl PostV1ReferenceEuVatRatesListResponseBuilder {
    pub fn notice(mut self, value: impl Into<String>) -> Self {
        self.notice = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReferenceEuVatRatesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notice`](PostV1ReferenceEuVatRatesListResponseBuilder::notice)
    /// - [`rows`](PostV1ReferenceEuVatRatesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesListResponse, BuildError> {
        Ok(PostV1ReferenceEuVatRatesListResponse {
            notice: self
                .notice
                .ok_or_else(|| BuildError::missing_field("notice"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
