pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GbCorporationTaxComputeDeclarationsResponsePartsItem {
    #[serde(rename = "financialYear")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub financial_year: f64,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub days: f64,
    #[serde(default)]
    pub profits: String,
    #[serde(rename = "augmentedProfits")]
    #[serde(default)]
    pub augmented_profits: String,
    #[serde(rename = "lowerLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<String>,
    #[serde(rename = "upperLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<String>,
    pub rate: GbCorporationTaxComputeDeclarationsResponsePartsItemRate,
    #[serde(rename = "taxAtMainRate")]
    #[serde(default)]
    pub tax_at_main_rate: String,
    #[serde(rename = "marginalRelief")]
    #[serde(default)]
    pub marginal_relief: String,
    #[serde(default)]
    pub tax: String,
}

impl GbCorporationTaxComputeDeclarationsResponsePartsItem {
    pub fn builder() -> GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder {
        <GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder {
    financial_year: Option<f64>,
    from: Option<String>,
    to: Option<String>,
    days: Option<f64>,
    profits: Option<String>,
    augmented_profits: Option<String>,
    lower_limit: Option<String>,
    upper_limit: Option<String>,
    rate: Option<GbCorporationTaxComputeDeclarationsResponsePartsItemRate>,
    tax_at_main_rate: Option<String>,
    marginal_relief: Option<String>,
    tax: Option<String>,
}

impl GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder {
    pub fn financial_year(mut self, value: f64) -> Self {
        self.financial_year = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn days(mut self, value: f64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn profits(mut self, value: impl Into<String>) -> Self {
        self.profits = Some(value.into());
        self
    }

    pub fn augmented_profits(mut self, value: impl Into<String>) -> Self {
        self.augmented_profits = Some(value.into());
        self
    }

    pub fn lower_limit(mut self, value: impl Into<String>) -> Self {
        self.lower_limit = Some(value.into());
        self
    }

    pub fn upper_limit(mut self, value: impl Into<String>) -> Self {
        self.upper_limit = Some(value.into());
        self
    }

    pub fn rate(mut self, value: GbCorporationTaxComputeDeclarationsResponsePartsItemRate) -> Self {
        self.rate = Some(value);
        self
    }

    pub fn tax_at_main_rate(mut self, value: impl Into<String>) -> Self {
        self.tax_at_main_rate = Some(value.into());
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

    /// Consumes the builder and constructs a [`GbCorporationTaxComputeDeclarationsResponsePartsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`financial_year`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::financial_year)
    /// - [`from`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::from)
    /// - [`to`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::to)
    /// - [`days`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::days)
    /// - [`profits`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::profits)
    /// - [`augmented_profits`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::augmented_profits)
    /// - [`rate`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::rate)
    /// - [`tax_at_main_rate`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::tax_at_main_rate)
    /// - [`marginal_relief`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::marginal_relief)
    /// - [`tax`](GbCorporationTaxComputeDeclarationsResponsePartsItemBuilder::tax)
    pub fn build(self) -> Result<GbCorporationTaxComputeDeclarationsResponsePartsItem, BuildError> {
        Ok(GbCorporationTaxComputeDeclarationsResponsePartsItem {
            financial_year: self
                .financial_year
                .ok_or_else(|| BuildError::missing_field("financial_year"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
            profits: self
                .profits
                .ok_or_else(|| BuildError::missing_field("profits"))?,
            augmented_profits: self
                .augmented_profits
                .ok_or_else(|| BuildError::missing_field("augmented_profits"))?,
            lower_limit: self.lower_limit,
            upper_limit: self.upper_limit,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            tax_at_main_rate: self
                .tax_at_main_rate
                .ok_or_else(|| BuildError::missing_field("tax_at_main_rate"))?,
            marginal_relief: self
                .marginal_relief
                .ok_or_else(|| BuildError::missing_field("marginal_relief"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
        })
    }
}
