pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPeppolSendResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "receiverId")]
    #[serde(default)]
    pub receiver_id: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
}

impl PostV1SalesInvoicesPeppolSendResponse {
    pub fn builder() -> PostV1SalesInvoicesPeppolSendResponseBuilder {
        <PostV1SalesInvoicesPeppolSendResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPeppolSendResponseBuilder {
    sent: Option<bool>,
    message_id: Option<String>,
    receiver_id: Option<String>,
    file_id: Option<String>,
}

impl PostV1SalesInvoicesPeppolSendResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn receiver_id(mut self, value: impl Into<String>) -> Self {
        self.receiver_id = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPeppolSendResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](PostV1SalesInvoicesPeppolSendResponseBuilder::sent)
    /// - [`message_id`](PostV1SalesInvoicesPeppolSendResponseBuilder::message_id)
    /// - [`receiver_id`](PostV1SalesInvoicesPeppolSendResponseBuilder::receiver_id)
    /// - [`file_id`](PostV1SalesInvoicesPeppolSendResponseBuilder::file_id)
    pub fn build(self) -> Result<PostV1SalesInvoicesPeppolSendResponse, BuildError> {
        Ok(PostV1SalesInvoicesPeppolSendResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            receiver_id: self
                .receiver_id
                .ok_or_else(|| BuildError::missing_field("receiver_id"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
