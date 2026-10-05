pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancialStatementsReportsResponseCashFlow {
    #[serde(rename = "openingCash")]
    #[serde(default)]
    pub opening_cash: String,
    #[serde(default)]
    pub operating: String,
    #[serde(default)]
    pub investing: String,
    #[serde(default)]
    pub financing: String,
    #[serde(rename = "netChange")]
    #[serde(default)]
    pub net_change: String,
    #[serde(rename = "closingCash")]
    #[serde(default)]
    pub closing_cash: String,
}

impl FinancialStatementsReportsResponseCashFlow {
    pub fn builder() -> FinancialStatementsReportsResponseCashFlowBuilder {
        <FinancialStatementsReportsResponseCashFlowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialStatementsReportsResponseCashFlowBuilder {
    opening_cash: Option<String>,
    operating: Option<String>,
    investing: Option<String>,
    financing: Option<String>,
    net_change: Option<String>,
    closing_cash: Option<String>,
}

impl FinancialStatementsReportsResponseCashFlowBuilder {
    pub fn opening_cash(mut self, value: impl Into<String>) -> Self {
        self.opening_cash = Some(value.into());
        self
    }

    pub fn operating(mut self, value: impl Into<String>) -> Self {
        self.operating = Some(value.into());
        self
    }

    pub fn investing(mut self, value: impl Into<String>) -> Self {
        self.investing = Some(value.into());
        self
    }

    pub fn financing(mut self, value: impl Into<String>) -> Self {
        self.financing = Some(value.into());
        self
    }

    pub fn net_change(mut self, value: impl Into<String>) -> Self {
        self.net_change = Some(value.into());
        self
    }

    pub fn closing_cash(mut self, value: impl Into<String>) -> Self {
        self.closing_cash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinancialStatementsReportsResponseCashFlow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`opening_cash`](FinancialStatementsReportsResponseCashFlowBuilder::opening_cash)
    /// - [`operating`](FinancialStatementsReportsResponseCashFlowBuilder::operating)
    /// - [`investing`](FinancialStatementsReportsResponseCashFlowBuilder::investing)
    /// - [`financing`](FinancialStatementsReportsResponseCashFlowBuilder::financing)
    /// - [`net_change`](FinancialStatementsReportsResponseCashFlowBuilder::net_change)
    /// - [`closing_cash`](FinancialStatementsReportsResponseCashFlowBuilder::closing_cash)
    pub fn build(self) -> Result<FinancialStatementsReportsResponseCashFlow, BuildError> {
        Ok(FinancialStatementsReportsResponseCashFlow {
            opening_cash: self
                .opening_cash
                .ok_or_else(|| BuildError::missing_field("opening_cash"))?,
            operating: self
                .operating
                .ok_or_else(|| BuildError::missing_field("operating"))?,
            investing: self
                .investing
                .ok_or_else(|| BuildError::missing_field("investing"))?,
            financing: self
                .financing
                .ok_or_else(|| BuildError::missing_field("financing"))?,
            net_change: self
                .net_change
                .ok_or_else(|| BuildError::missing_field("net_change"))?,
            closing_cash: self
                .closing_cash
                .ok_or_else(|| BuildError::missing_field("closing_cash"))?,
        })
    }
}
