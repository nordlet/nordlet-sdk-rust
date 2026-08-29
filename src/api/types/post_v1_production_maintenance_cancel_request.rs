pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionMaintenanceCancelRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProductionMaintenanceCancelRequest {
    pub fn builder() -> PostV1ProductionMaintenanceCancelRequestBuilder {
        <PostV1ProductionMaintenanceCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionMaintenanceCancelRequestBuilder {
    id: Option<String>,
}

impl PostV1ProductionMaintenanceCancelRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionMaintenanceCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionMaintenanceCancelRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProductionMaintenanceCancelRequest, BuildError> {
        Ok(PostV1ProductionMaintenanceCancelRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
