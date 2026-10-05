pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuVatRatesListReferenceResponseRowsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub category: EuVatRatesListReferenceResponseRowsItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    pub source: EuVatRatesListReferenceResponseRowsItemSource,
}

impl EuVatRatesListReferenceResponseRowsItem {
    pub fn builder() -> EuVatRatesListReferenceResponseRowsItemBuilder {
        <EuVatRatesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesListReferenceResponseRowsItemBuilder {
    country_code: Option<String>,
    category: Option<EuVatRatesListReferenceResponseRowsItemCategory>,
    rate_percent: Option<String>,
    valid_from: Option<String>,
    valid_to: Option<String>,
    source: Option<EuVatRatesListReferenceResponseRowsItemSource>,
}

impl EuVatRatesListReferenceResponseRowsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn category(mut self, value: EuVatRatesListReferenceResponseRowsItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    pub fn valid_from(mut self, value: impl Into<String>) -> Self {
        self.valid_from = Some(value.into());
        self
    }

    pub fn valid_to(mut self, value: impl Into<String>) -> Self {
        self.valid_to = Some(value.into());
        self
    }

    pub fn source(mut self, value: EuVatRatesListReferenceResponseRowsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuVatRatesListReferenceResponseRowsItemBuilder::country_code)
    /// - [`category`](EuVatRatesListReferenceResponseRowsItemBuilder::category)
    /// - [`rate_percent`](EuVatRatesListReferenceResponseRowsItemBuilder::rate_percent)
    /// - [`source`](EuVatRatesListReferenceResponseRowsItemBuilder::source)
    pub fn build(self) -> Result<EuVatRatesListReferenceResponseRowsItem, BuildError> {
        Ok(EuVatRatesListReferenceResponseRowsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
