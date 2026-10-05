pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesEinvoiceXmlSalesResponse {
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

impl InvoicesEinvoiceXmlSalesResponse {
    pub fn builder() -> InvoicesEinvoiceXmlSalesResponseBuilder {
        <InvoicesEinvoiceXmlSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesEinvoiceXmlSalesResponseBuilder {
    format: Option<String>,
    system: Option<String>,
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    warnings: Option<Vec<String>>,
}

impl InvoicesEinvoiceXmlSalesResponseBuilder {
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

    /// Consumes the builder and constructs a [`InvoicesEinvoiceXmlSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](InvoicesEinvoiceXmlSalesResponseBuilder::format)
    /// - [`system`](InvoicesEinvoiceXmlSalesResponseBuilder::system)
    /// - [`file_name`](InvoicesEinvoiceXmlSalesResponseBuilder::file_name)
    /// - [`content_type`](InvoicesEinvoiceXmlSalesResponseBuilder::content_type)
    /// - [`data`](InvoicesEinvoiceXmlSalesResponseBuilder::data)
    /// - [`warnings`](InvoicesEinvoiceXmlSalesResponseBuilder::warnings)
    pub fn build(self) -> Result<InvoicesEinvoiceXmlSalesResponse, BuildError> {
        Ok(InvoicesEinvoiceXmlSalesResponse {
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
