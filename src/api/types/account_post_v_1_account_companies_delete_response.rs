pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesDeleteResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "purgeAfter")]
    #[serde(default)]
    pub purge_after: String,
}

impl PostV1AccountCompaniesDeleteResponse {
    pub fn builder() -> PostV1AccountCompaniesDeleteResponseBuilder {
        <PostV1AccountCompaniesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesDeleteResponseBuilder {
    id: Option<String>,
    status: Option<String>,
    purge_after: Option<String>,
}

impl PostV1AccountCompaniesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn purge_after(mut self, value: impl Into<String>) -> Self {
        self.purge_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountCompaniesDeleteResponseBuilder::id)
    /// - [`status`](PostV1AccountCompaniesDeleteResponseBuilder::status)
    /// - [`purge_after`](PostV1AccountCompaniesDeleteResponseBuilder::purge_after)
    pub fn build(self) -> Result<PostV1AccountCompaniesDeleteResponse, BuildError> {
        Ok(PostV1AccountCompaniesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            purge_after: self
                .purge_after
                .ok_or_else(|| BuildError::missing_field("purge_after"))?,
        })
    }
}
