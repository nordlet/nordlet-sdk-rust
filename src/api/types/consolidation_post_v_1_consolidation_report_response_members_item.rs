pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseMembersItem {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(rename = "ownershipPercent")]
    #[serde(default)]
    pub ownership_percent: String,
    pub method: PostV1ConsolidationReportResponseMembersItemMethod,
    #[serde(rename = "fxFactor")]
    #[serde(default)]
    pub fx_factor: String,
    #[serde(rename = "rateFrom")]
    #[serde(default)]
    pub rate_from: String,
    #[serde(rename = "rateTo")]
    #[serde(default)]
    pub rate_to: String,
    #[serde(rename = "totalAssets")]
    #[serde(default)]
    pub total_assets: String,
    #[serde(rename = "netEquity")]
    #[serde(default)]
    pub net_equity: String,
    #[serde(rename = "periodResult")]
    #[serde(default)]
    pub period_result: String,
}

impl PostV1ConsolidationReportResponseMembersItem {
    pub fn builder() -> PostV1ConsolidationReportResponseMembersItemBuilder {
        <PostV1ConsolidationReportResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseMembersItemBuilder {
    company_id: Option<String>,
    name: Option<String>,
    base_currency: Option<String>,
    ownership_percent: Option<String>,
    method: Option<PostV1ConsolidationReportResponseMembersItemMethod>,
    fx_factor: Option<String>,
    rate_from: Option<String>,
    rate_to: Option<String>,
    total_assets: Option<String>,
    net_equity: Option<String>,
    period_result: Option<String>,
}

impl PostV1ConsolidationReportResponseMembersItemBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn ownership_percent(mut self, value: impl Into<String>) -> Self {
        self.ownership_percent = Some(value.into());
        self
    }

    pub fn method(mut self, value: PostV1ConsolidationReportResponseMembersItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn fx_factor(mut self, value: impl Into<String>) -> Self {
        self.fx_factor = Some(value.into());
        self
    }

    pub fn rate_from(mut self, value: impl Into<String>) -> Self {
        self.rate_from = Some(value.into());
        self
    }

    pub fn rate_to(mut self, value: impl Into<String>) -> Self {
        self.rate_to = Some(value.into());
        self
    }

    pub fn total_assets(mut self, value: impl Into<String>) -> Self {
        self.total_assets = Some(value.into());
        self
    }

    pub fn net_equity(mut self, value: impl Into<String>) -> Self {
        self.net_equity = Some(value.into());
        self
    }

    pub fn period_result(mut self, value: impl Into<String>) -> Self {
        self.period_result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseMembersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](PostV1ConsolidationReportResponseMembersItemBuilder::company_id)
    /// - [`name`](PostV1ConsolidationReportResponseMembersItemBuilder::name)
    /// - [`base_currency`](PostV1ConsolidationReportResponseMembersItemBuilder::base_currency)
    /// - [`ownership_percent`](PostV1ConsolidationReportResponseMembersItemBuilder::ownership_percent)
    /// - [`method`](PostV1ConsolidationReportResponseMembersItemBuilder::method)
    /// - [`fx_factor`](PostV1ConsolidationReportResponseMembersItemBuilder::fx_factor)
    /// - [`rate_from`](PostV1ConsolidationReportResponseMembersItemBuilder::rate_from)
    /// - [`rate_to`](PostV1ConsolidationReportResponseMembersItemBuilder::rate_to)
    /// - [`total_assets`](PostV1ConsolidationReportResponseMembersItemBuilder::total_assets)
    /// - [`net_equity`](PostV1ConsolidationReportResponseMembersItemBuilder::net_equity)
    /// - [`period_result`](PostV1ConsolidationReportResponseMembersItemBuilder::period_result)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseMembersItem, BuildError> {
        Ok(PostV1ConsolidationReportResponseMembersItem {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            ownership_percent: self
                .ownership_percent
                .ok_or_else(|| BuildError::missing_field("ownership_percent"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            fx_factor: self
                .fx_factor
                .ok_or_else(|| BuildError::missing_field("fx_factor"))?,
            rate_from: self
                .rate_from
                .ok_or_else(|| BuildError::missing_field("rate_from"))?,
            rate_to: self
                .rate_to
                .ok_or_else(|| BuildError::missing_field("rate_to"))?,
            total_assets: self
                .total_assets
                .ok_or_else(|| BuildError::missing_field("total_assets"))?,
            net_equity: self
                .net_equity
                .ok_or_else(|| BuildError::missing_field("net_equity"))?,
            period_result: self
                .period_result
                .ok_or_else(|| BuildError::missing_field("period_result"))?,
        })
    }
}
