pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionComputeSalesResponse {
    #[serde(rename = "asOfDate")]
    #[serde(default)]
    pub as_of_date: NaiveDate,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: String,
    #[serde(default)]
    pub rows: Vec<RecognitionComputeSalesResponseRowsItem>,
}

impl RecognitionComputeSalesResponse {
    pub fn builder() -> RecognitionComputeSalesResponseBuilder {
        <RecognitionComputeSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionComputeSalesResponseBuilder {
    as_of_date: Option<NaiveDate>,
    total_amount: Option<String>,
    rows: Option<Vec<RecognitionComputeSalesResponseRowsItem>>,
}

impl RecognitionComputeSalesResponseBuilder {
    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    pub fn total_amount(mut self, value: impl Into<String>) -> Self {
        self.total_amount = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<RecognitionComputeSalesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionComputeSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of_date`](RecognitionComputeSalesResponseBuilder::as_of_date)
    /// - [`total_amount`](RecognitionComputeSalesResponseBuilder::total_amount)
    /// - [`rows`](RecognitionComputeSalesResponseBuilder::rows)
    pub fn build(self) -> Result<RecognitionComputeSalesResponse, BuildError> {
        Ok(RecognitionComputeSalesResponse {
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
