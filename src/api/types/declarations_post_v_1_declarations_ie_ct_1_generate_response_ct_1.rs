pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeCt1GenerateResponseCt1 {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
}

impl PostV1DeclarationsIeCt1GenerateResponseCt1 {
    pub fn builder() -> PostV1DeclarationsIeCt1GenerateResponseCt1Builder {
        <PostV1DeclarationsIeCt1GenerateResponseCt1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeCt1GenerateResponseCt1Builder {
    file_name: Option<String>,
    xml: Option<String>,
}

impl PostV1DeclarationsIeCt1GenerateResponseCt1Builder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeCt1GenerateResponseCt1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsIeCt1GenerateResponseCt1Builder::file_name)
    /// - [`xml`](PostV1DeclarationsIeCt1GenerateResponseCt1Builder::xml)
    pub fn build(self) -> Result<PostV1DeclarationsIeCt1GenerateResponseCt1, BuildError> {
        Ok(PostV1DeclarationsIeCt1GenerateResponseCt1 {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
