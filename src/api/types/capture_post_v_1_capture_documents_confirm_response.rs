pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsConfirmResponse {
    pub capture: PostV1CaptureDocumentsConfirmResponseCapture,
    pub invoice: PostV1CaptureDocumentsConfirmResponseInvoice,
}

impl PostV1CaptureDocumentsConfirmResponse {
    pub fn builder() -> PostV1CaptureDocumentsConfirmResponseBuilder {
        <PostV1CaptureDocumentsConfirmResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsConfirmResponseBuilder {
    capture: Option<PostV1CaptureDocumentsConfirmResponseCapture>,
    invoice: Option<PostV1CaptureDocumentsConfirmResponseInvoice>,
}

impl PostV1CaptureDocumentsConfirmResponseBuilder {
    pub fn capture(mut self, value: PostV1CaptureDocumentsConfirmResponseCapture) -> Self {
        self.capture = Some(value);
        self
    }

    pub fn invoice(mut self, value: PostV1CaptureDocumentsConfirmResponseInvoice) -> Self {
        self.invoice = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsConfirmResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capture`](PostV1CaptureDocumentsConfirmResponseBuilder::capture)
    /// - [`invoice`](PostV1CaptureDocumentsConfirmResponseBuilder::invoice)
    pub fn build(self) -> Result<PostV1CaptureDocumentsConfirmResponse, BuildError> {
        Ok(PostV1CaptureDocumentsConfirmResponse {
            capture: self
                .capture
                .ok_or_else(|| BuildError::missing_field("capture"))?,
            invoice: self
                .invoice
                .ok_or_else(|| BuildError::missing_field("invoice"))?,
        })
    }
}
