pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeCt1GenerateResponseAccounts {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xhtml: String,
}

impl PostV1DeclarationsIeCt1GenerateResponseAccounts {
    pub fn builder() -> PostV1DeclarationsIeCt1GenerateResponseAccountsBuilder {
        <PostV1DeclarationsIeCt1GenerateResponseAccountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeCt1GenerateResponseAccountsBuilder {
    file_name: Option<String>,
    xhtml: Option<String>,
}

impl PostV1DeclarationsIeCt1GenerateResponseAccountsBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xhtml(mut self, value: impl Into<String>) -> Self {
        self.xhtml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeCt1GenerateResponseAccounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsIeCt1GenerateResponseAccountsBuilder::file_name)
    /// - [`xhtml`](PostV1DeclarationsIeCt1GenerateResponseAccountsBuilder::xhtml)
    pub fn build(self) -> Result<PostV1DeclarationsIeCt1GenerateResponseAccounts, BuildError> {
        Ok(PostV1DeclarationsIeCt1GenerateResponseAccounts {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xhtml: self
                .xhtml
                .ok_or_else(|| BuildError::missing_field("xhtml"))?,
        })
    }
}
