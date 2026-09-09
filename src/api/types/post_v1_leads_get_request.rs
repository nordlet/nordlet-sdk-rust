pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsGetRequest {
    pub fn builder() -> PostV1LeadsGetRequestBuilder {
        <PostV1LeadsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1LeadsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsGetRequest, BuildError> {
        Ok(PostV1LeadsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
