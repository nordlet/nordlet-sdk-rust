pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse {
    pub fn builder() -> PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder {
        <PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder {
    file_name: Option<String>,
    mime_type: Option<String>,
    content: Option<String>,
    text: Option<String>,
    source: Option<String>,
}

impl PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder {
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

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder::file_name)
    /// - [`mime_type`](PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder::mime_type)
    /// - [`content`](PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder::content)
    /// - [`text`](PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder::text)
    /// - [`source`](PostV1DeclarationsEsVerifactuDeclaracionResponsableResponseBuilder::source)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse, BuildError> {
        Ok(
            PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse {
                file_name: self
                    .file_name
                    .ok_or_else(|| BuildError::missing_field("file_name"))?,
                mime_type: self
                    .mime_type
                    .ok_or_else(|| BuildError::missing_field("mime_type"))?,
                content: self
                    .content
                    .ok_or_else(|| BuildError::missing_field("content"))?,
                text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
                source: self
                    .source
                    .ok_or_else(|| BuildError::missing_field("source"))?,
            },
        )
    }
}
