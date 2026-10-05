pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CashFlowReportsResponseFinancing {
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub rows: Vec<CashFlowReportsResponseFinancingRowsItem>,
}

impl CashFlowReportsResponseFinancing {
    pub fn builder() -> CashFlowReportsResponseFinancingBuilder {
        <CashFlowReportsResponseFinancingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashFlowReportsResponseFinancingBuilder {
    inflow: Option<String>,
    outflow: Option<String>,
    net: Option<String>,
    rows: Option<Vec<CashFlowReportsResponseFinancingRowsItem>>,
}

impl CashFlowReportsResponseFinancingBuilder {
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

    pub fn rows(mut self, value: Vec<CashFlowReportsResponseFinancingRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CashFlowReportsResponseFinancing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inflow`](CashFlowReportsResponseFinancingBuilder::inflow)
    /// - [`outflow`](CashFlowReportsResponseFinancingBuilder::outflow)
    /// - [`net`](CashFlowReportsResponseFinancingBuilder::net)
    /// - [`rows`](CashFlowReportsResponseFinancingBuilder::rows)
    pub fn build(self) -> Result<CashFlowReportsResponseFinancing, BuildError> {
        Ok(CashFlowReportsResponseFinancing {
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
