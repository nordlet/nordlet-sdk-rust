pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionProgressRequest {
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    #[serde(rename = "percentComplete")]
    #[serde(default)]
    pub percent_complete: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1SalesRecognitionProgressRequest {
    pub fn builder() -> PostV1SalesRecognitionProgressRequestBuilder {
        <PostV1SalesRecognitionProgressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionProgressRequestBuilder {
    invoice_line_id: Option<String>,
    percent_complete: Option<String>,
    date: Option<String>,
}

impl PostV1SalesRecognitionProgressRequestBuilder {
    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn percent_complete(mut self, value: impl Into<String>) -> Self {
        self.percent_complete = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionProgressRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_line_id`](PostV1SalesRecognitionProgressRequestBuilder::invoice_line_id)
    /// - [`percent_complete`](PostV1SalesRecognitionProgressRequestBuilder::percent_complete)
    pub fn build(self) -> Result<PostV1SalesRecognitionProgressRequest, BuildError> {
        Ok(PostV1SalesRecognitionProgressRequest {
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
