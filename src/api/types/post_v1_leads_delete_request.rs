pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsDeleteRequest {
    pub fn builder() -> PostV1LeadsDeleteRequestBuilder {
        <PostV1LeadsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1LeadsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsDeleteRequest, BuildError> {
        Ok(PostV1LeadsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
