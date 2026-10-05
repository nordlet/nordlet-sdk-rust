pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCancelPayrollResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl RunsCancelPayrollResponse {
    pub fn builder() -> RunsCancelPayrollResponseBuilder {
        <RunsCancelPayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCancelPayrollResponseBuilder {
    deleted: Option<bool>,
}

impl RunsCancelPayrollResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunsCancelPayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](RunsCancelPayrollResponseBuilder::deleted)
    pub fn build(self) -> Result<RunsCancelPayrollResponse, BuildError> {
        Ok(RunsCancelPayrollResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
