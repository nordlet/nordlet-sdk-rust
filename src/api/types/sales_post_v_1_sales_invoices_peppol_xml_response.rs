pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPeppolXmlResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(rename = "senderId")]
    #[serde(default)]
    pub sender_id: String,
    #[serde(rename = "receiverId")]
    #[serde(default)]
    pub receiver_id: String,
}

impl PostV1SalesInvoicesPeppolXmlResponse {
    pub fn builder() -> PostV1SalesInvoicesPeppolXmlResponseBuilder {
        <PostV1SalesInvoicesPeppolXmlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPeppolXmlResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    sender_id: Option<String>,
    receiver_id: Option<String>,
}

impl PostV1SalesInvoicesPeppolXmlResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn sender_id(mut self, value: impl Into<String>) -> Self {
        self.sender_id = Some(value.into());
        self
    }

    pub fn receiver_id(mut self, value: impl Into<String>) -> Self {
        self.receiver_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPeppolXmlResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1SalesInvoicesPeppolXmlResponseBuilder::file_name)
    /// - [`content_type`](PostV1SalesInvoicesPeppolXmlResponseBuilder::content_type)
    /// - [`data`](PostV1SalesInvoicesPeppolXmlResponseBuilder::data)
    /// - [`sender_id`](PostV1SalesInvoicesPeppolXmlResponseBuilder::sender_id)
    /// - [`receiver_id`](PostV1SalesInvoicesPeppolXmlResponseBuilder::receiver_id)
    pub fn build(self) -> Result<PostV1SalesInvoicesPeppolXmlResponse, BuildError> {
        Ok(PostV1SalesInvoicesPeppolXmlResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            sender_id: self
                .sender_id
                .ok_or_else(|| BuildError::missing_field("sender_id"))?,
            receiver_id: self
                .receiver_id
                .ok_or_else(|| BuildError::missing_field("receiver_id"))?,
        })
    }
}
