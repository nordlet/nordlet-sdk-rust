pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesArchiveResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl PostV1AccountCompaniesArchiveResponse {
    pub fn builder() -> PostV1AccountCompaniesArchiveResponseBuilder {
        <PostV1AccountCompaniesArchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesArchiveResponseBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl PostV1AccountCompaniesArchiveResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesArchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountCompaniesArchiveResponseBuilder::id)
    /// - [`status`](PostV1AccountCompaniesArchiveResponseBuilder::status)
    pub fn build(self) -> Result<PostV1AccountCompaniesArchiveResponse, BuildError> {
        Ok(PostV1AccountCompaniesArchiveResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
