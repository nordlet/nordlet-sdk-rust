pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseCashFlowFinancing {
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub rows: Vec<ReportConsolidationResponseCashFlowFinancingRowsItem>,
}

impl ReportConsolidationResponseCashFlowFinancing {
    pub fn builder() -> ReportConsolidationResponseCashFlowFinancingBuilder {
        <ReportConsolidationResponseCashFlowFinancingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseCashFlowFinancingBuilder {
    inflow: Option<String>,
    outflow: Option<String>,
    net: Option<String>,
    rows: Option<Vec<ReportConsolidationResponseCashFlowFinancingRowsItem>>,
}

impl ReportConsolidationResponseCashFlowFinancingBuilder {
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
        value: Vec<ReportConsolidationResponseCashFlowFinancingRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseCashFlowFinancing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inflow`](ReportConsolidationResponseCashFlowFinancingBuilder::inflow)
    /// - [`outflow`](ReportConsolidationResponseCashFlowFinancingBuilder::outflow)
    /// - [`net`](ReportConsolidationResponseCashFlowFinancingBuilder::net)
    /// - [`rows`](ReportConsolidationResponseCashFlowFinancingBuilder::rows)
    pub fn build(self) -> Result<ReportConsolidationResponseCashFlowFinancing, BuildError> {
        Ok(ReportConsolidationResponseCashFlowFinancing {
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
