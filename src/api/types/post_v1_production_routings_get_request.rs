pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionRoutingsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProductionRoutingsGetRequest {
    pub fn builder() -> PostV1ProductionRoutingsGetRequestBuilder {
        <PostV1ProductionRoutingsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionRoutingsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1ProductionRoutingsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionRoutingsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionRoutingsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProductionRoutingsGetRequest, BuildError> {
        Ok(PostV1ProductionRoutingsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
