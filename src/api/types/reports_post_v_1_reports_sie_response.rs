pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsSieResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub accounts: i64,
    #[serde(default)]
    pub vouchers: i64,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1ReportsSieResponse {
    pub fn builder() -> PostV1ReportsSieResponseBuilder {
        <PostV1ReportsSieResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsSieResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    accounts: Option<i64>,
    vouchers: Option<i64>,
    source: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1ReportsSieResponseBuilder {
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

    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn vouchers(mut self, value: i64) -> Self {
        self.vouchers = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsSieResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1ReportsSieResponseBuilder::file_name)
    /// - [`content_type`](PostV1ReportsSieResponseBuilder::content_type)
    /// - [`data`](PostV1ReportsSieResponseBuilder::data)
    /// - [`accounts`](PostV1ReportsSieResponseBuilder::accounts)
    /// - [`vouchers`](PostV1ReportsSieResponseBuilder::vouchers)
    /// - [`source`](PostV1ReportsSieResponseBuilder::source)
    /// - [`warnings`](PostV1ReportsSieResponseBuilder::warnings)
    /// - [`notes`](PostV1ReportsSieResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1ReportsSieResponse, BuildError> {
        Ok(PostV1ReportsSieResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            vouchers: self
                .vouchers
                .ok_or_else(|| BuildError::missing_field("vouchers"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
