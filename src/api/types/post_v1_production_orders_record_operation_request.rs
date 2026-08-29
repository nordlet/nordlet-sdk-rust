pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersRecordOperationRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "actualMinutes")]
    #[serde(default)]
    pub actual_minutes: String,
}

impl PostV1ProductionOrdersRecordOperationRequest {
    pub fn builder() -> PostV1ProductionOrdersRecordOperationRequestBuilder {
        <PostV1ProductionOrdersRecordOperationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersRecordOperationRequestBuilder {
    id: Option<String>,
    actual_minutes: Option<String>,
}

impl PostV1ProductionOrdersRecordOperationRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn actual_minutes(mut self, value: impl Into<String>) -> Self {
        self.actual_minutes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersRecordOperationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionOrdersRecordOperationRequestBuilder::id)
    /// - [`actual_minutes`](PostV1ProductionOrdersRecordOperationRequestBuilder::actual_minutes)
    pub fn build(self) -> Result<PostV1ProductionOrdersRecordOperationRequest, BuildError> {
        Ok(PostV1ProductionOrdersRecordOperationRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            actual_minutes: self
                .actual_minutes
                .ok_or_else(|| BuildError::missing_field("actual_minutes"))?,
        })
    }
}
