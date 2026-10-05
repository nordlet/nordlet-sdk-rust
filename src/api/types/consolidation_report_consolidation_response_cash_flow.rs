pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseCashFlow {
    #[serde(rename = "openingCash")]
    #[serde(default)]
    pub opening_cash: String,
    #[serde(rename = "closingCash")]
    #[serde(default)]
    pub closing_cash: String,
    #[serde(rename = "netChange")]
    #[serde(default)]
    pub net_change: String,
    #[serde(default)]
    pub operating: ReportConsolidationResponseCashFlowOperating,
    #[serde(default)]
    pub investing: ReportConsolidationResponseCashFlowInvesting,
    #[serde(default)]
    pub financing: ReportConsolidationResponseCashFlowFinancing,
    #[serde(default)]
    pub balanced: bool,
}

impl ReportConsolidationResponseCashFlow {
    pub fn builder() -> ReportConsolidationResponseCashFlowBuilder {
        <ReportConsolidationResponseCashFlowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseCashFlowBuilder {
    opening_cash: Option<String>,
    closing_cash: Option<String>,
    net_change: Option<String>,
    operating: Option<ReportConsolidationResponseCashFlowOperating>,
    investing: Option<ReportConsolidationResponseCashFlowInvesting>,
    financing: Option<ReportConsolidationResponseCashFlowFinancing>,
    balanced: Option<bool>,
}

impl ReportConsolidationResponseCashFlowBuilder {
    pub fn opening_cash(mut self, value: impl Into<String>) -> Self {
        self.opening_cash = Some(value.into());
        self
    }

    pub fn closing_cash(mut self, value: impl Into<String>) -> Self {
        self.closing_cash = Some(value.into());
        self
    }

    pub fn net_change(mut self, value: impl Into<String>) -> Self {
        self.net_change = Some(value.into());
        self
    }

    pub fn operating(mut self, value: ReportConsolidationResponseCashFlowOperating) -> Self {
        self.operating = Some(value);
        self
    }

    pub fn investing(mut self, value: ReportConsolidationResponseCashFlowInvesting) -> Self {
        self.investing = Some(value);
        self
    }

    pub fn financing(mut self, value: ReportConsolidationResponseCashFlowFinancing) -> Self {
        self.financing = Some(value);
        self
    }

    pub fn balanced(mut self, value: bool) -> Self {
        self.balanced = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseCashFlow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`opening_cash`](ReportConsolidationResponseCashFlowBuilder::opening_cash)
    /// - [`closing_cash`](ReportConsolidationResponseCashFlowBuilder::closing_cash)
    /// - [`net_change`](ReportConsolidationResponseCashFlowBuilder::net_change)
    /// - [`operating`](ReportConsolidationResponseCashFlowBuilder::operating)
    /// - [`investing`](ReportConsolidationResponseCashFlowBuilder::investing)
    /// - [`financing`](ReportConsolidationResponseCashFlowBuilder::financing)
    /// - [`balanced`](ReportConsolidationResponseCashFlowBuilder::balanced)
    pub fn build(self) -> Result<ReportConsolidationResponseCashFlow, BuildError> {
        Ok(ReportConsolidationResponseCashFlow {
            opening_cash: self
                .opening_cash
                .ok_or_else(|| BuildError::missing_field("opening_cash"))?,
            closing_cash: self
                .closing_cash
                .ok_or_else(|| BuildError::missing_field("closing_cash"))?,
            net_change: self
                .net_change
                .ok_or_else(|| BuildError::missing_field("net_change"))?,
            operating: self
                .operating
                .ok_or_else(|| BuildError::missing_field("operating"))?,
            investing: self
                .investing
                .ok_or_else(|| BuildError::missing_field("investing"))?,
            financing: self
                .financing
                .ok_or_else(|| BuildError::missing_field("financing"))?,
            balanced: self
                .balanced
                .ok_or_else(|| BuildError::missing_field("balanced"))?,
        })
    }
}
