pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsSourcesDeleteRequest {
    pub fn builder() -> PostV1LeadsSourcesDeleteRequestBuilder {
        <PostV1LeadsSourcesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1LeadsSourcesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsSourcesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsSourcesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsSourcesDeleteRequest, BuildError> {
        Ok(PostV1LeadsSourcesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
