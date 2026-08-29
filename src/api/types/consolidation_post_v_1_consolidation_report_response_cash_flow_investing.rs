pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseCashFlowInvesting {
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub rows: Vec<PostV1ConsolidationReportResponseCashFlowInvestingRowsItem>,
}

impl PostV1ConsolidationReportResponseCashFlowInvesting {
    pub fn builder() -> PostV1ConsolidationReportResponseCashFlowInvestingBuilder {
        <PostV1ConsolidationReportResponseCashFlowInvestingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseCashFlowInvestingBuilder {
    inflow: Option<String>,
    outflow: Option<String>,
    net: Option<String>,
    rows: Option<Vec<PostV1ConsolidationReportResponseCashFlowInvestingRowsItem>>,
}

impl PostV1ConsolidationReportResponseCashFlowInvestingBuilder {
    pub fn inflow(mut self, value: impl Into<String>) -> Self {
        self.inflow = Some(value.into());
        self
    }

    pub fn outflow(mut self, value: impl Into<String>) -> Self {
        self.outflow = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1ConsolidationReportResponseCashFlowInvestingRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseCashFlowInvesting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inflow`](PostV1ConsolidationReportResponseCashFlowInvestingBuilder::inflow)
    /// - [`outflow`](PostV1ConsolidationReportResponseCashFlowInvestingBuilder::outflow)
    /// - [`net`](PostV1ConsolidationReportResponseCashFlowInvestingBuilder::net)
    /// - [`rows`](PostV1ConsolidationReportResponseCashFlowInvestingBuilder::rows)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseCashFlowInvesting, BuildError> {
        Ok(PostV1ConsolidationReportResponseCashFlowInvesting {
            inflow: self
                .inflow
                .ok_or_else(|| BuildError::missing_field("inflow"))?,
            outflow: self
                .outflow
                .ok_or_else(|| BuildError::missing_field("outflow"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
