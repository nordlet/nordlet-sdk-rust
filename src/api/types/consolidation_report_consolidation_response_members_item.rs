pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseMembersItem {
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
    pub method: ReportConsolidationResponseMembersItemMethod,
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

impl ReportConsolidationResponseMembersItem {
    pub fn builder() -> ReportConsolidationResponseMembersItemBuilder {
        <ReportConsolidationResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseMembersItemBuilder {
    company_id: Option<String>,
    name: Option<String>,
    base_currency: Option<String>,
    ownership_percent: Option<String>,
    method: Option<ReportConsolidationResponseMembersItemMethod>,
    fx_factor: Option<String>,
    rate_from: Option<String>,
    rate_to: Option<String>,
    total_assets: Option<String>,
    net_equity: Option<String>,
    period_result: Option<String>,
}

impl ReportConsolidationResponseMembersItemBuilder {
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

    pub fn method(mut self, value: ReportConsolidationResponseMembersItemMethod) -> Self {
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

    /// Consumes the builder and constructs a [`ReportConsolidationResponseMembersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](ReportConsolidationResponseMembersItemBuilder::company_id)
    /// - [`name`](ReportConsolidationResponseMembersItemBuilder::name)
    /// - [`base_currency`](ReportConsolidationResponseMembersItemBuilder::base_currency)
    /// - [`ownership_percent`](ReportConsolidationResponseMembersItemBuilder::ownership_percent)
    /// - [`method`](ReportConsolidationResponseMembersItemBuilder::method)
    /// - [`fx_factor`](ReportConsolidationResponseMembersItemBuilder::fx_factor)
    /// - [`rate_from`](ReportConsolidationResponseMembersItemBuilder::rate_from)
    /// - [`rate_to`](ReportConsolidationResponseMembersItemBuilder::rate_to)
    /// - [`total_assets`](ReportConsolidationResponseMembersItemBuilder::total_assets)
    /// - [`net_equity`](ReportConsolidationResponseMembersItemBuilder::net_equity)
    /// - [`period_result`](ReportConsolidationResponseMembersItemBuilder::period_result)
    pub fn build(self) -> Result<ReportConsolidationResponseMembersItem, BuildError> {
        Ok(ReportConsolidationResponseMembersItem {
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
