pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProductionBomsGetRequest {
    pub fn builder() -> PostV1ProductionBomsGetRequestBuilder {
        <PostV1ProductionBomsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1ProductionBomsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionBomsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProductionBomsGetRequest, BuildError> {
        Ok(PostV1ProductionBomsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
