pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionProgressSalesRequest {
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    #[serde(rename = "percentComplete")]
    #[serde(default)]
    pub percent_complete: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl RecognitionProgressSalesRequest {
    pub fn builder() -> RecognitionProgressSalesRequestBuilder {
        <RecognitionProgressSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionProgressSalesRequestBuilder {
    invoice_line_id: Option<String>,
    percent_complete: Option<String>,
    date: Option<NaiveDate>,
}

impl RecognitionProgressSalesRequestBuilder {
    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn percent_complete(mut self, value: impl Into<String>) -> Self {
        self.percent_complete = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionProgressSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_line_id`](RecognitionProgressSalesRequestBuilder::invoice_line_id)
    /// - [`percent_complete`](RecognitionProgressSalesRequestBuilder::percent_complete)
    pub fn build(self) -> Result<RecognitionProgressSalesRequest, BuildError> {
        Ok(RecognitionProgressSalesRequest {
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            percent_complete: self
                .percent_complete
                .ok_or_else(|| BuildError::missing_field("percent_complete"))?,
            date: self.date,
        })
    }
}
