pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProjectsGetRequest {
    pub fn builder() -> PostV1ProjectsGetRequestBuilder {
        <PostV1ProjectsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1ProjectsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProjectsGetRequest, BuildError> {
        Ok(PostV1ProjectsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
