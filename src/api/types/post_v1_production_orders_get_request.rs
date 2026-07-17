pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProductionOrdersGetRequest {
    pub fn builder() -> PostV1ProductionOrdersGetRequestBuilder {
        <PostV1ProductionOrdersGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersGetRequestBuilder {
    id: Option<String>,
}

impl PostV1ProductionOrdersGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionOrdersGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProductionOrdersGetRequest, BuildError> {
        Ok(PostV1ProductionOrdersGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
