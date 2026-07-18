pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionComputeResponse {
    #[serde(rename = "asOfDate")]
    #[serde(default)]
    pub as_of_date: String,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: String,
    #[serde(default)]
    pub rows: Vec<PostV1SalesRecognitionComputeResponseRowsItem>,
}

impl PostV1SalesRecognitionComputeResponse {
    pub fn builder() -> PostV1SalesRecognitionComputeResponseBuilder {
        <PostV1SalesRecognitionComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionComputeResponseBuilder {
    as_of_date: Option<String>,
    total_amount: Option<String>,
    rows: Option<Vec<PostV1SalesRecognitionComputeResponseRowsItem>>,
}

impl PostV1SalesRecognitionComputeResponseBuilder {
    pub fn as_of_date(mut self, value: impl Into<String>) -> Self {
        self.as_of_date = Some(value.into());
        self
    }

    pub fn total_amount(mut self, value: impl Into<String>) -> Self {
        self.total_amount = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1SalesRecognitionComputeResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of_date`](PostV1SalesRecognitionComputeResponseBuilder::as_of_date)
    /// - [`total_amount`](PostV1SalesRecognitionComputeResponseBuilder::total_amount)
    /// - [`rows`](PostV1SalesRecognitionComputeResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1SalesRecognitionComputeResponse, BuildError> {
        Ok(PostV1SalesRecognitionComputeResponse {
            as_of_date: self
                .as_of_date
                .ok_or_else(|| BuildError::missing_field("as_of_date"))?,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
