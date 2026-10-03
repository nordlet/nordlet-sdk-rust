pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnsGenerateResponse {
    #[serde(rename = "ruleKey")]
    #[serde(default)]
    pub rule_key: String,
    #[serde(default)]
    pub period: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsDeReturnsGenerateResponse {
    pub fn builder() -> PostV1DeclarationsDeReturnsGenerateResponseBuilder {
        <PostV1DeclarationsDeReturnsGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnsGenerateResponseBuilder {
    rule_key: Option<String>,
    period: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    variant: Option<String>,
    content: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsDeReturnsGenerateResponseBuilder {
    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnsGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](PostV1DeclarationsDeReturnsGenerateResponseBuilder::rule_key)
    /// - [`period`](PostV1DeclarationsDeReturnsGenerateResponseBuilder::period)
    /// - [`file_name`](PostV1DeclarationsDeReturnsGenerateResponseBuilder::file_name)
    /// - [`mime_type`](PostV1DeclarationsDeReturnsGenerateResponseBuilder::mime_type)
    /// - [`content`](PostV1DeclarationsDeReturnsGenerateResponseBuilder::content)
    /// - [`warnings`](PostV1DeclarationsDeReturnsGenerateResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnsGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsDeReturnsGenerateResponse {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            variant: self.variant,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
