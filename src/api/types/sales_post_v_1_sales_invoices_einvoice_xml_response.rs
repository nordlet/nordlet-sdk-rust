pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesEinvoiceXmlResponse {
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub system: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1SalesInvoicesEinvoiceXmlResponse {
    pub fn builder() -> PostV1SalesInvoicesEinvoiceXmlResponseBuilder {
        <PostV1SalesInvoicesEinvoiceXmlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesEinvoiceXmlResponseBuilder {
    format: Option<String>,
    system: Option<String>,
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1SalesInvoicesEinvoiceXmlResponseBuilder {
    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesEinvoiceXmlResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](PostV1SalesInvoicesEinvoiceXmlResponseBuilder::format)
    /// - [`system`](PostV1SalesInvoicesEinvoiceXmlResponseBuilder::system)
    /// - [`file_name`](PostV1SalesInvoicesEinvoiceXmlResponseBuilder::file_name)
    /// - [`content_type`](PostV1SalesInvoicesEinvoiceXmlResponseBuilder::content_type)
    /// - [`data`](PostV1SalesInvoicesEinvoiceXmlResponseBuilder::data)
    /// - [`warnings`](PostV1SalesInvoicesEinvoiceXmlResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1SalesInvoicesEinvoiceXmlResponse, BuildError> {
        Ok(PostV1SalesInvoicesEinvoiceXmlResponse {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
