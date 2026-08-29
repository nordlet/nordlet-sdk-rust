pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseCashFlow {
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
    pub operating: PostV1ConsolidationReportResponseCashFlowOperating,
    #[serde(default)]
    pub investing: PostV1ConsolidationReportResponseCashFlowInvesting,
    #[serde(default)]
    pub financing: PostV1ConsolidationReportResponseCashFlowFinancing,
    #[serde(default)]
    pub balanced: bool,
}

impl PostV1ConsolidationReportResponseCashFlow {
    pub fn builder() -> PostV1ConsolidationReportResponseCashFlowBuilder {
        <PostV1ConsolidationReportResponseCashFlowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseCashFlowBuilder {
    opening_cash: Option<String>,
    closing_cash: Option<String>,
    net_change: Option<String>,
    operating: Option<PostV1ConsolidationReportResponseCashFlowOperating>,
    investing: Option<PostV1ConsolidationReportResponseCashFlowInvesting>,
    financing: Option<PostV1ConsolidationReportResponseCashFlowFinancing>,
    balanced: Option<bool>,
}

impl PostV1ConsolidationReportResponseCashFlowBuilder {
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

    pub fn operating(mut self, value: PostV1ConsolidationReportResponseCashFlowOperating) -> Self {
        self.operating = Some(value);
        self
    }

    pub fn investing(mut self, value: PostV1ConsolidationReportResponseCashFlowInvesting) -> Self {
        self.investing = Some(value);
        self
    }

    pub fn financing(mut self, value: PostV1ConsolidationReportResponseCashFlowFinancing) -> Self {
        self.financing = Some(value);
        self
    }

    pub fn balanced(mut self, value: bool) -> Self {
        self.balanced = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseCashFlow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`opening_cash`](PostV1ConsolidationReportResponseCashFlowBuilder::opening_cash)
    /// - [`closing_cash`](PostV1ConsolidationReportResponseCashFlowBuilder::closing_cash)
    /// - [`net_change`](PostV1ConsolidationReportResponseCashFlowBuilder::net_change)
    /// - [`operating`](PostV1ConsolidationReportResponseCashFlowBuilder::operating)
    /// - [`investing`](PostV1ConsolidationReportResponseCashFlowBuilder::investing)
    /// - [`financing`](PostV1ConsolidationReportResponseCashFlowBuilder::financing)
    /// - [`balanced`](PostV1ConsolidationReportResponseCashFlowBuilder::balanced)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseCashFlow, BuildError> {
        Ok(PostV1ConsolidationReportResponseCashFlow {
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
