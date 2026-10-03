pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OfficersDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1OfficersDeleteResponse {
    pub fn builder() -> PostV1OfficersDeleteResponseBuilder {
        <PostV1OfficersDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OfficersDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1OfficersDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1OfficersDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OfficersDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1OfficersDeleteResponse, BuildError> {
        Ok(PostV1OfficersDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
