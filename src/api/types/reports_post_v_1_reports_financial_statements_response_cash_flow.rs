pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsResponseCashFlow {
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

impl PostV1ReportsFinancialStatementsResponseCashFlow {
    pub fn builder() -> PostV1ReportsFinancialStatementsResponseCashFlowBuilder {
        <PostV1ReportsFinancialStatementsResponseCashFlowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsResponseCashFlowBuilder {
    opening_cash: Option<String>,
    operating: Option<String>,
    investing: Option<String>,
    financing: Option<String>,
    net_change: Option<String>,
    closing_cash: Option<String>,
}

impl PostV1ReportsFinancialStatementsResponseCashFlowBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsResponseCashFlow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`opening_cash`](PostV1ReportsFinancialStatementsResponseCashFlowBuilder::opening_cash)
    /// - [`operating`](PostV1ReportsFinancialStatementsResponseCashFlowBuilder::operating)
    /// - [`investing`](PostV1ReportsFinancialStatementsResponseCashFlowBuilder::investing)
    /// - [`financing`](PostV1ReportsFinancialStatementsResponseCashFlowBuilder::financing)
    /// - [`net_change`](PostV1ReportsFinancialStatementsResponseCashFlowBuilder::net_change)
    /// - [`closing_cash`](PostV1ReportsFinancialStatementsResponseCashFlowBuilder::closing_cash)
    pub fn build(self) -> Result<PostV1ReportsFinancialStatementsResponseCashFlow, BuildError> {
        Ok(PostV1ReportsFinancialStatementsResponseCashFlow {
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
