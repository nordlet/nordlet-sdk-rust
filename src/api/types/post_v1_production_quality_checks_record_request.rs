pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProductionQualityChecksRecordRequest {
    #[serde(default)]
    pub id: String,
    pub result: PostV1ProductionQualityChecksRecordRequestResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionQualityChecksRecordRequest {
    pub fn builder() -> PostV1ProductionQualityChecksRecordRequestBuilder {
        <PostV1ProductionQualityChecksRecordRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionQualityChecksRecordRequestBuilder {
    id: Option<String>,
    result: Option<PostV1ProductionQualityChecksRecordRequestResult>,
    notes: Option<String>,
}

impl PostV1ProductionQualityChecksRecordRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn result(mut self, value: PostV1ProductionQualityChecksRecordRequestResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionQualityChecksRecordRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionQualityChecksRecordRequestBuilder::id)
    /// - [`result`](PostV1ProductionQualityChecksRecordRequestBuilder::result)
    pub fn build(self) -> Result<PostV1ProductionQualityChecksRecordRequest, BuildError> {
        Ok(PostV1ProductionQualityChecksRecordRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            notes: self.notes,
        })
    }
}
