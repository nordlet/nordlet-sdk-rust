pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesEinvoiceSendResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1SalesInvoicesEinvoiceSendResponse {
    pub fn builder() -> PostV1SalesInvoicesEinvoiceSendResponseBuilder {
        <PostV1SalesInvoicesEinvoiceSendResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesEinvoiceSendResponseBuilder {
    sent: Option<bool>,
    system: Option<String>,
    format: Option<String>,
    message_id: Option<String>,
    file_id: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1SalesInvoicesEinvoiceSendResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesEinvoiceSendResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](PostV1SalesInvoicesEinvoiceSendResponseBuilder::sent)
    /// - [`system`](PostV1SalesInvoicesEinvoiceSendResponseBuilder::system)
    /// - [`format`](PostV1SalesInvoicesEinvoiceSendResponseBuilder::format)
    /// - [`message_id`](PostV1SalesInvoicesEinvoiceSendResponseBuilder::message_id)
    /// - [`file_id`](PostV1SalesInvoicesEinvoiceSendResponseBuilder::file_id)
    /// - [`warnings`](PostV1SalesInvoicesEinvoiceSendResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1SalesInvoicesEinvoiceSendResponse, BuildError> {
        Ok(PostV1SalesInvoicesEinvoiceSendResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
