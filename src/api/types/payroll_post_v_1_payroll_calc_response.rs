pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollCalcResponse {
    #[serde(default)]
    pub npd: String,
    #[serde(default)]
    pub gpm: String,
    #[serde(rename = "sodraEmployee")]
    #[serde(default)]
    pub sodra_employee: String,
    #[serde(rename = "sodraEmployer")]
    #[serde(default)]
    pub sodra_employer: String,
    #[serde(default)]
    pub net: String,
}

impl PostV1PayrollCalcResponse {
    pub fn builder() -> PostV1PayrollCalcResponseBuilder {
        <PostV1PayrollCalcResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollCalcResponseBuilder {
    npd: Option<String>,
    gpm: Option<String>,
    sodra_employee: Option<String>,
    sodra_employer: Option<String>,
    net: Option<String>,
}

impl PostV1PayrollCalcResponseBuilder {
    pub fn npd(mut self, value: impl Into<String>) -> Self {
        self.npd = Some(value.into());
        self
    }

    pub fn gpm(mut self, value: impl Into<String>) -> Self {
        self.gpm = Some(value.into());
        self
    }

    pub fn sodra_employee(mut self, value: impl Into<String>) -> Self {
        self.sodra_employee = Some(value.into());
        self
    }

    pub fn sodra_employer(mut self, value: impl Into<String>) -> Self {
        self.sodra_employer = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollCalcResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`npd`](PostV1PayrollCalcResponseBuilder::npd)
    /// - [`gpm`](PostV1PayrollCalcResponseBuilder::gpm)
    /// - [`sodra_employee`](PostV1PayrollCalcResponseBuilder::sodra_employee)
    /// - [`sodra_employer`](PostV1PayrollCalcResponseBuilder::sodra_employer)
    /// - [`net`](PostV1PayrollCalcResponseBuilder::net)
    pub fn build(self) -> Result<PostV1PayrollCalcResponse, BuildError> {
        Ok(PostV1PayrollCalcResponse {
            npd: self.npd.ok_or_else(|| BuildError::missing_field("npd"))?,
            gpm: self.gpm.ok_or_else(|| BuildError::missing_field("gpm"))?,
            sodra_employee: self
                .sodra_employee
                .ok_or_else(|| BuildError::missing_field("sodra_employee"))?,
            sodra_employer: self
                .sodra_employer
                .ok_or_else(|| BuildError::missing_field("sodra_employer"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
