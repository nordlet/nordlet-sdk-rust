pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDac7XmlDeclarationsResponse {
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

impl EuDac7XmlDeclarationsResponse {
    pub fn builder() -> EuDac7XmlDeclarationsResponseBuilder {
        <EuDac7XmlDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDac7XmlDeclarationsResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    warnings: Option<Vec<String>>,
}

impl EuDac7XmlDeclarationsResponseBuilder {
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

    /// Consumes the builder and constructs a [`EuDac7XmlDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](EuDac7XmlDeclarationsResponseBuilder::file_name)
    /// - [`content_type`](EuDac7XmlDeclarationsResponseBuilder::content_type)
    /// - [`data`](EuDac7XmlDeclarationsResponseBuilder::data)
    /// - [`warnings`](EuDac7XmlDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<EuDac7XmlDeclarationsResponse, BuildError> {
        Ok(EuDac7XmlDeclarationsResponse {
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
