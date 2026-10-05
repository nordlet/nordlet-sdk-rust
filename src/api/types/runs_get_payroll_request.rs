pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsGetPayrollRequest {
    #[serde(default)]
    pub id: String,
}

impl RunsGetPayrollRequest {
    pub fn builder() -> RunsGetPayrollRequestBuilder {
        <RunsGetPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsGetPayrollRequestBuilder {
    id: Option<String>,
}

impl RunsGetPayrollRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsGetPayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RunsGetPayrollRequestBuilder::id)
    pub fn build(self) -> Result<RunsGetPayrollRequest, BuildError> {
        Ok(RunsGetPayrollRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
