pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuVatRatesSetOverridesReferenceResponseRowsItem {
    pub category: EuVatRatesSetOverridesReferenceResponseRowsItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
}

impl EuVatRatesSetOverridesReferenceResponseRowsItem {
    pub fn builder() -> EuVatRatesSetOverridesReferenceResponseRowsItemBuilder {
        <EuVatRatesSetOverridesReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesSetOverridesReferenceResponseRowsItemBuilder {
    category: Option<EuVatRatesSetOverridesReferenceResponseRowsItemCategory>,
    rate_percent: Option<String>,
}

impl EuVatRatesSetOverridesReferenceResponseRowsItemBuilder {
    pub fn category(
        mut self,
        value: EuVatRatesSetOverridesReferenceResponseRowsItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesSetOverridesReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](EuVatRatesSetOverridesReferenceResponseRowsItemBuilder::category)
    /// - [`rate_percent`](EuVatRatesSetOverridesReferenceResponseRowsItemBuilder::rate_percent)
    pub fn build(self) -> Result<EuVatRatesSetOverridesReferenceResponseRowsItem, BuildError> {
        Ok(EuVatRatesSetOverridesReferenceResponseRowsItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
        })
    }
}
