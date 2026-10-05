pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QualityChecksRecordProductionRequest {
    #[serde(default)]
    pub id: String,
    pub result: QualityChecksRecordProductionRequestResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl QualityChecksRecordProductionRequest {
    pub fn builder() -> QualityChecksRecordProductionRequestBuilder {
        <QualityChecksRecordProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QualityChecksRecordProductionRequestBuilder {
    id: Option<String>,
    result: Option<QualityChecksRecordProductionRequestResult>,
    notes: Option<String>,
}

impl QualityChecksRecordProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn result(mut self, value: QualityChecksRecordProductionRequestResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QualityChecksRecordProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QualityChecksRecordProductionRequestBuilder::id)
    /// - [`result`](QualityChecksRecordProductionRequestBuilder::result)
    pub fn build(self) -> Result<QualityChecksRecordProductionRequest, BuildError> {
        Ok(QualityChecksRecordProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            notes: self.notes,
        })
    }
}
