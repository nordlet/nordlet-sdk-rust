pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCashFlowResponseInvestingRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
}

impl PostV1ReportsCashFlowResponseInvestingRowsItem {
    pub fn builder() -> PostV1ReportsCashFlowResponseInvestingRowsItemBuilder {
        <PostV1ReportsCashFlowResponseInvestingRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCashFlowResponseInvestingRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    inflow: Option<String>,
    outflow: Option<String>,
}

impl PostV1ReportsCashFlowResponseInvestingRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn inflow(mut self, value: impl Into<String>) -> Self {
        self.inflow = Some(value.into());
        self
    }

    pub fn outflow(mut self, value: impl Into<String>) -> Self {
        self.outflow = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCashFlowResponseInvestingRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReportsCashFlowResponseInvestingRowsItemBuilder::code)
    /// - [`name`](PostV1ReportsCashFlowResponseInvestingRowsItemBuilder::name)
    /// - [`inflow`](PostV1ReportsCashFlowResponseInvestingRowsItemBuilder::inflow)
    /// - [`outflow`](PostV1ReportsCashFlowResponseInvestingRowsItemBuilder::outflow)
    pub fn build(self) -> Result<PostV1ReportsCashFlowResponseInvestingRowsItem, BuildError> {
        Ok(PostV1ReportsCashFlowResponseInvestingRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            inflow: self
                .inflow
                .ok_or_else(|| BuildError::missing_field("inflow"))?,
            outflow: self
                .outflow
                .ok_or_else(|| BuildError::missing_field("outflow"))?,
        })
    }
}
