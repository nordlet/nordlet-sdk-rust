pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCancelPayrollRequest {
    #[serde(default)]
    pub id: String,
}

impl RunsCancelPayrollRequest {
    pub fn builder() -> RunsCancelPayrollRequestBuilder {
        <RunsCancelPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCancelPayrollRequestBuilder {
    id: Option<String>,
}

impl RunsCancelPayrollRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsCancelPayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RunsCancelPayrollRequestBuilder::id)
    pub fn build(self) -> Result<RunsCancelPayrollRequest, BuildError> {
        Ok(RunsCancelPayrollRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
