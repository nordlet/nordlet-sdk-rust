pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbCorporationTaxComputeDeclarationsRequest {
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: NaiveDate,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: NaiveDate,
    #[serde(rename = "taxableTotalProfits")]
    #[serde(default)]
    pub taxable_total_profits: String,
    #[serde(rename = "exemptDistributions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_distributions: Option<String>,
    #[serde(rename = "associatedCompanies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_companies: Option<i64>,
    #[serde(rename = "closeInvestmentHolding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_investment_holding: Option<bool>,
}

impl GbCorporationTaxComputeDeclarationsRequest {
    pub fn builder() -> GbCorporationTaxComputeDeclarationsRequestBuilder {
        <GbCorporationTaxComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbCorporationTaxComputeDeclarationsRequestBuilder {
    period_start: Option<NaiveDate>,
    period_end: Option<NaiveDate>,
    taxable_total_profits: Option<String>,
    exempt_distributions: Option<String>,
    associated_companies: Option<i64>,
    close_investment_holding: Option<bool>,
}

impl GbCorporationTaxComputeDeclarationsRequestBuilder {
    pub fn period_start(mut self, value: NaiveDate) -> Self {
        self.period_start = Some(value);
        self
    }

    pub fn period_end(mut self, value: NaiveDate) -> Self {
        self.period_end = Some(value);
        self
    }

    pub fn taxable_total_profits(mut self, value: impl Into<String>) -> Self {
        self.taxable_total_profits = Some(value.into());
        self
    }

    pub fn exempt_distributions(mut self, value: impl Into<String>) -> Self {
        self.exempt_distributions = Some(value.into());
        self
    }

    pub fn associated_companies(mut self, value: i64) -> Self {
        self.associated_companies = Some(value);
        self
    }

    pub fn close_investment_holding(mut self, value: bool) -> Self {
        self.close_investment_holding = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbCorporationTaxComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_start`](GbCorporationTaxComputeDeclarationsRequestBuilder::period_start)
    /// - [`period_end`](GbCorporationTaxComputeDeclarationsRequestBuilder::period_end)
    /// - [`taxable_total_profits`](GbCorporationTaxComputeDeclarationsRequestBuilder::taxable_total_profits)
    pub fn build(self) -> Result<GbCorporationTaxComputeDeclarationsRequest, BuildError> {
        Ok(GbCorporationTaxComputeDeclarationsRequest {
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            taxable_total_profits: self
                .taxable_total_profits
                .ok_or_else(|| BuildError::missing_field("taxable_total_profits"))?,
            exempt_distributions: self.exempt_distributions,
            associated_companies: self.associated_companies,
            close_investment_holding: self.close_investment_holding,
        })
    }
}
