pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OfficersDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1OfficersDeleteRequest {
    pub fn builder() -> PostV1OfficersDeleteRequestBuilder {
        <PostV1OfficersDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OfficersDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1OfficersDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1OfficersDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OfficersDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1OfficersDeleteRequest, BuildError> {
        Ok(PostV1OfficersDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
