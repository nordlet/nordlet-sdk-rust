pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbCorporationTaxComputeDeclarationsResponse {
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub days: f64,
    #[serde(rename = "taxableTotalProfits")]
    #[serde(default)]
    pub taxable_total_profits: String,
    #[serde(rename = "augmentedProfits")]
    #[serde(default)]
    pub augmented_profits: String,
    #[serde(default)]
    pub parts: Vec<GbCorporationTaxComputeDeclarationsResponsePartsItem>,
    #[serde(rename = "marginalRelief")]
    #[serde(default)]
    pub marginal_relief: String,
    #[serde(default)]
    pub tax: String,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl GbCorporationTaxComputeDeclarationsResponse {
    pub fn builder() -> GbCorporationTaxComputeDeclarationsResponseBuilder {
        <GbCorporationTaxComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbCorporationTaxComputeDeclarationsResponseBuilder {
    period_start: Option<String>,
    period_end: Option<String>,
    days: Option<f64>,
    taxable_total_profits: Option<String>,
    augmented_profits: Option<String>,
    parts: Option<Vec<GbCorporationTaxComputeDeclarationsResponsePartsItem>>,
    marginal_relief: Option<String>,
    tax: Option<String>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl GbCorporationTaxComputeDeclarationsResponseBuilder {
    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn days(mut self, value: f64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn taxable_total_profits(mut self, value: impl Into<String>) -> Self {
        self.taxable_total_profits = Some(value.into());
        self
    }

    pub fn augmented_profits(mut self, value: impl Into<String>) -> Self {
        self.augmented_profits = Some(value.into());
        self
    }

    pub fn parts(
        mut self,
        value: Vec<GbCorporationTaxComputeDeclarationsResponsePartsItem>,
    ) -> Self {
        self.parts = Some(value);
        self
    }

    pub fn marginal_relief(mut self, value: impl Into<String>) -> Self {
        self.marginal_relief = Some(value.into());
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbCorporationTaxComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_start`](GbCorporationTaxComputeDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](GbCorporationTaxComputeDeclarationsResponseBuilder::period_end)
    /// - [`days`](GbCorporationTaxComputeDeclarationsResponseBuilder::days)
    /// - [`taxable_total_profits`](GbCorporationTaxComputeDeclarationsResponseBuilder::taxable_total_profits)
    /// - [`augmented_profits`](GbCorporationTaxComputeDeclarationsResponseBuilder::augmented_profits)
    /// - [`parts`](GbCorporationTaxComputeDeclarationsResponseBuilder::parts)
    /// - [`marginal_relief`](GbCorporationTaxComputeDeclarationsResponseBuilder::marginal_relief)
    /// - [`tax`](GbCorporationTaxComputeDeclarationsResponseBuilder::tax)
    /// - [`notes`](GbCorporationTaxComputeDeclarationsResponseBuilder::notes)
    /// - [`source`](GbCorporationTaxComputeDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<GbCorporationTaxComputeDeclarationsResponse, BuildError> {
        Ok(GbCorporationTaxComputeDeclarationsResponse {
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
            taxable_total_profits: self
                .taxable_total_profits
                .ok_or_else(|| BuildError::missing_field("taxable_total_profits"))?,
            augmented_profits: self
                .augmented_profits
                .ok_or_else(|| BuildError::missing_field("augmented_profits"))?,
            parts: self
                .parts
                .ok_or_else(|| BuildError::missing_field("parts"))?,
            marginal_relief: self
                .marginal_relief
                .ok_or_else(|| BuildError::missing_field("marginal_relief"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
