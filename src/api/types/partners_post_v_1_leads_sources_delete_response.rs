pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsSourcesDeleteResponse {
    pub fn builder() -> PostV1LeadsSourcesDeleteResponseBuilder {
        <PostV1LeadsSourcesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1LeadsSourcesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsSourcesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsSourcesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsSourcesDeleteResponse, BuildError> {
        Ok(PostV1LeadsSourcesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
