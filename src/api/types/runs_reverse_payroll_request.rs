pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsReversePayrollRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reason: String,
}

impl RunsReversePayrollRequest {
    pub fn builder() -> RunsReversePayrollRequestBuilder {
        <RunsReversePayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsReversePayrollRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl RunsReversePayrollRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsReversePayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RunsReversePayrollRequestBuilder::id)
    /// - [`reason`](RunsReversePayrollRequestBuilder::reason)
    pub fn build(self) -> Result<RunsReversePayrollRequest, BuildError> {
        Ok(RunsReversePayrollRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
