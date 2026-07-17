pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsPdfResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
}

impl PostV1SalesActsPdfResponse {
    pub fn builder() -> PostV1SalesActsPdfResponseBuilder {
        <PostV1SalesActsPdfResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsPdfResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
}

impl PostV1SalesActsPdfResponseBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesActsPdfResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1SalesActsPdfResponseBuilder::file_name)
    /// - [`content_type`](PostV1SalesActsPdfResponseBuilder::content_type)
    /// - [`data`](PostV1SalesActsPdfResponseBuilder::data)
    pub fn build(self) -> Result<PostV1SalesActsPdfResponse, BuildError> {
        Ok(PostV1SalesActsPdfResponse {
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
