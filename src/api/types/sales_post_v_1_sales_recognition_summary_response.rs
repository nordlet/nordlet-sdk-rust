pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionSummaryResponse {
    #[serde(default)]
    pub rows: Vec<PostV1SalesRecognitionSummaryResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1SalesRecognitionSummaryResponseTotals,
}

impl PostV1SalesRecognitionSummaryResponse {
    pub fn builder() -> PostV1SalesRecognitionSummaryResponseBuilder {
        <PostV1SalesRecognitionSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionSummaryResponseBuilder {
    rows: Option<Vec<PostV1SalesRecognitionSummaryResponseRowsItem>>,
    totals: Option<PostV1SalesRecognitionSummaryResponseTotals>,
}

impl PostV1SalesRecognitionSummaryResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1SalesRecognitionSummaryResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1SalesRecognitionSummaryResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1SalesRecognitionSummaryResponseBuilder::rows)
    /// - [`totals`](PostV1SalesRecognitionSummaryResponseBuilder::totals)
    pub fn build(self) -> Result<PostV1SalesRecognitionSummaryResponse, BuildError> {
        Ok(PostV1SalesRecognitionSummaryResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
