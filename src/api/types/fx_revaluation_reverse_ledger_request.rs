pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FxRevaluationReverseLedgerRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reason: String,
}

impl FxRevaluationReverseLedgerRequest {
    pub fn builder() -> FxRevaluationReverseLedgerRequestBuilder {
        <FxRevaluationReverseLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FxRevaluationReverseLedgerRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl FxRevaluationReverseLedgerRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FxRevaluationReverseLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FxRevaluationReverseLedgerRequestBuilder::id)
    /// - [`reason`](FxRevaluationReverseLedgerRequestBuilder::reason)
    pub fn build(self) -> Result<FxRevaluationReverseLedgerRequest, BuildError> {
        Ok(FxRevaluationReverseLedgerRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
