pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentsConfirmCaptureResponse {
    pub capture: DocumentsConfirmCaptureResponseCapture,
    pub invoice: DocumentsConfirmCaptureResponseInvoice,
}

impl DocumentsConfirmCaptureResponse {
    pub fn builder() -> DocumentsConfirmCaptureResponseBuilder {
        <DocumentsConfirmCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsConfirmCaptureResponseBuilder {
    capture: Option<DocumentsConfirmCaptureResponseCapture>,
    invoice: Option<DocumentsConfirmCaptureResponseInvoice>,
}

impl DocumentsConfirmCaptureResponseBuilder {
    pub fn capture(mut self, value: DocumentsConfirmCaptureResponseCapture) -> Self {
        self.capture = Some(value);
        self
    }

    pub fn invoice(mut self, value: DocumentsConfirmCaptureResponseInvoice) -> Self {
        self.invoice = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsConfirmCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capture`](DocumentsConfirmCaptureResponseBuilder::capture)
    /// - [`invoice`](DocumentsConfirmCaptureResponseBuilder::invoice)
    pub fn build(self) -> Result<DocumentsConfirmCaptureResponse, BuildError> {
        Ok(DocumentsConfirmCaptureResponse {
            capture: self
                .capture
                .ok_or_else(|| BuildError::missing_field("capture"))?,
            invoice: self
                .invoice
                .ok_or_else(|| BuildError::missing_field("invoice"))?,
        })
    }
}
