pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCashFlowResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
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
    pub operating: PostV1ReportsCashFlowResponseOperating,
    #[serde(default)]
    pub investing: PostV1ReportsCashFlowResponseInvesting,
    #[serde(default)]
    pub financing: PostV1ReportsCashFlowResponseFinancing,
    #[serde(default)]
    pub balanced: bool,
}

impl PostV1ReportsCashFlowResponse {
    pub fn builder() -> PostV1ReportsCashFlowResponseBuilder {
        <PostV1ReportsCashFlowResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCashFlowResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    opening_cash: Option<String>,
    closing_cash: Option<String>,
    net_change: Option<String>,
    operating: Option<PostV1ReportsCashFlowResponseOperating>,
    investing: Option<PostV1ReportsCashFlowResponseInvesting>,
    financing: Option<PostV1ReportsCashFlowResponseFinancing>,
    balanced: Option<bool>,
}

impl PostV1ReportsCashFlowResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
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

    pub fn operating(mut self, value: PostV1ReportsCashFlowResponseOperating) -> Self {
        self.operating = Some(value);
        self
    }

    pub fn investing(mut self, value: PostV1ReportsCashFlowResponseInvesting) -> Self {
        self.investing = Some(value);
        self
    }

    pub fn financing(mut self, value: PostV1ReportsCashFlowResponseFinancing) -> Self {
        self.financing = Some(value);
        self
    }

    pub fn balanced(mut self, value: bool) -> Self {
        self.balanced = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCashFlowResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsCashFlowResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsCashFlowResponseBuilder::to_date)
    /// - [`opening_cash`](PostV1ReportsCashFlowResponseBuilder::opening_cash)
    /// - [`closing_cash`](PostV1ReportsCashFlowResponseBuilder::closing_cash)
    /// - [`net_change`](PostV1ReportsCashFlowResponseBuilder::net_change)
    /// - [`operating`](PostV1ReportsCashFlowResponseBuilder::operating)
    /// - [`investing`](PostV1ReportsCashFlowResponseBuilder::investing)
    /// - [`financing`](PostV1ReportsCashFlowResponseBuilder::financing)
    /// - [`balanced`](PostV1ReportsCashFlowResponseBuilder::balanced)
    pub fn build(self) -> Result<PostV1ReportsCashFlowResponse, BuildError> {
        Ok(PostV1ReportsCashFlowResponse {
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
