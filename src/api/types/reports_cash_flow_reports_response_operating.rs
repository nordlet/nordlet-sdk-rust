pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CashFlowReportsResponseOperating {
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub rows: Vec<CashFlowReportsResponseOperatingRowsItem>,
}

impl CashFlowReportsResponseOperating {
    pub fn builder() -> CashFlowReportsResponseOperatingBuilder {
        <CashFlowReportsResponseOperatingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashFlowReportsResponseOperatingBuilder {
    inflow: Option<String>,
    outflow: Option<String>,
    net: Option<String>,
    rows: Option<Vec<CashFlowReportsResponseOperatingRowsItem>>,
}

impl CashFlowReportsResponseOperatingBuilder {
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

    pub fn rows(mut self, value: Vec<CashFlowReportsResponseOperatingRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CashFlowReportsResponseOperating`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inflow`](CashFlowReportsResponseOperatingBuilder::inflow)
    /// - [`outflow`](CashFlowReportsResponseOperatingBuilder::outflow)
    /// - [`net`](CashFlowReportsResponseOperatingBuilder::net)
    /// - [`rows`](CashFlowReportsResponseOperatingBuilder::rows)
    pub fn build(self) -> Result<CashFlowReportsResponseOperating, BuildError> {
        Ok(CashFlowReportsResponseOperating {
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
