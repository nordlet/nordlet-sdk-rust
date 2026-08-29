pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseCashFlowInvestingRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
}

impl PostV1ConsolidationReportResponseCashFlowInvestingRowsItem {
    pub fn builder() -> PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder {
        <PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    inflow: Option<String>,
    outflow: Option<String>,
}

impl PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseCashFlowInvestingRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder::code)
    /// - [`name`](PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder::name)
    /// - [`inflow`](PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder::inflow)
    /// - [`outflow`](PostV1ConsolidationReportResponseCashFlowInvestingRowsItemBuilder::outflow)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationReportResponseCashFlowInvestingRowsItem, BuildError> {
        Ok(PostV1ConsolidationReportResponseCashFlowInvestingRowsItem {
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
