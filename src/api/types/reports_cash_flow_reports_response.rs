pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CashFlowReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
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
    pub operating: CashFlowReportsResponseOperating,
    #[serde(default)]
    pub investing: CashFlowReportsResponseInvesting,
    #[serde(default)]
    pub financing: CashFlowReportsResponseFinancing,
    #[serde(default)]
    pub balanced: bool,
}

impl CashFlowReportsResponse {
    pub fn builder() -> CashFlowReportsResponseBuilder {
        <CashFlowReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashFlowReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    opening_cash: Option<String>,
    closing_cash: Option<String>,
    net_change: Option<String>,
    operating: Option<CashFlowReportsResponseOperating>,
    investing: Option<CashFlowReportsResponseInvesting>,
    financing: Option<CashFlowReportsResponseFinancing>,
    balanced: Option<bool>,
}

impl CashFlowReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

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

    pub fn operating(mut self, value: CashFlowReportsResponseOperating) -> Self {
        self.operating = Some(value);
        self
    }

    pub fn investing(mut self, value: CashFlowReportsResponseInvesting) -> Self {
        self.investing = Some(value);
        self
    }

    pub fn financing(mut self, value: CashFlowReportsResponseFinancing) -> Self {
        self.financing = Some(value);
        self
    }

    pub fn balanced(mut self, value: bool) -> Self {
        self.balanced = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CashFlowReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](CashFlowReportsResponseBuilder::from_date)
    /// - [`to_date`](CashFlowReportsResponseBuilder::to_date)
    /// - [`opening_cash`](CashFlowReportsResponseBuilder::opening_cash)
    /// - [`closing_cash`](CashFlowReportsResponseBuilder::closing_cash)
    /// - [`net_change`](CashFlowReportsResponseBuilder::net_change)
    /// - [`operating`](CashFlowReportsResponseBuilder::operating)
    /// - [`investing`](CashFlowReportsResponseBuilder::investing)
    /// - [`financing`](CashFlowReportsResponseBuilder::financing)
    /// - [`balanced`](CashFlowReportsResponseBuilder::balanced)
    pub fn build(self) -> Result<CashFlowReportsResponse, BuildError> {
        Ok(CashFlowReportsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
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
