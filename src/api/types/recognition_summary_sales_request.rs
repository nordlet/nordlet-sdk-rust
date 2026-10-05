pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionSummarySalesRequest {
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
}

impl RecognitionSummarySalesRequest {
    pub fn builder() -> RecognitionSummarySalesRequestBuilder {
        <RecognitionSummarySalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSummarySalesRequestBuilder {
    invoice_id: Option<String>,
}

impl RecognitionSummarySalesRequestBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSummarySalesRequest`].
    pub fn build(self) -> Result<RecognitionSummarySalesRequest, BuildError> {
        Ok(RecognitionSummarySalesRequest {
            invoice_id: self.invoice_id,
        })
    }
}
