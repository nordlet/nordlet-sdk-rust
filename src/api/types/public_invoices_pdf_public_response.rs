pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPdfPublicResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
}

impl InvoicesPdfPublicResponse {
    pub fn builder() -> InvoicesPdfPublicResponseBuilder {
        <InvoicesPdfPublicResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPdfPublicResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
}

impl InvoicesPdfPublicResponseBuilder {
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

    /// Consumes the builder and constructs a [`InvoicesPdfPublicResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](InvoicesPdfPublicResponseBuilder::file_name)
    /// - [`content_type`](InvoicesPdfPublicResponseBuilder::content_type)
    /// - [`data`](InvoicesPdfPublicResponseBuilder::data)
    pub fn build(self) -> Result<InvoicesPdfPublicResponse, BuildError> {
        Ok(InvoicesPdfPublicResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
