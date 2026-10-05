pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesUpdateAccountRequestLogo {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    /// Base64-encoded image
    #[serde(default)]
    pub content: String,
}

impl CompaniesUpdateAccountRequestLogo {
    pub fn builder() -> CompaniesUpdateAccountRequestLogoBuilder {
        <CompaniesUpdateAccountRequestLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesUpdateAccountRequestLogoBuilder {
    file_name: Option<String>,
    mime_type: Option<String>,
    content: Option<String>,
}

impl CompaniesUpdateAccountRequestLogoBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesUpdateAccountRequestLogo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](CompaniesUpdateAccountRequestLogoBuilder::file_name)
    /// - [`mime_type`](CompaniesUpdateAccountRequestLogoBuilder::mime_type)
    /// - [`content`](CompaniesUpdateAccountRequestLogoBuilder::content)
    pub fn build(self) -> Result<CompaniesUpdateAccountRequestLogo, BuildError> {
        Ok(CompaniesUpdateAccountRequestLogo {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
