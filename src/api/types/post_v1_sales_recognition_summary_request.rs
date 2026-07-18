pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionSummaryRequest {
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
}

impl PostV1SalesRecognitionSummaryRequest {
    pub fn builder() -> PostV1SalesRecognitionSummaryRequestBuilder {
        <PostV1SalesRecognitionSummaryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionSummaryRequestBuilder {
    invoice_id: Option<String>,
}

impl PostV1SalesRecognitionSummaryRequestBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionSummaryRequest`].
    pub fn build(self) -> Result<PostV1SalesRecognitionSummaryRequest, BuildError> {
        Ok(PostV1SalesRecognitionSummaryRequest {
            invoice_id: self.invoice_id,
        })
    }
}
