pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PosReportsGetRequest {
    pub fn builder() -> PostV1PosReportsGetRequestBuilder {
        <PostV1PosReportsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PosReportsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosReportsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PosReportsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PosReportsGetRequest, BuildError> {
        Ok(PostV1PosReportsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
