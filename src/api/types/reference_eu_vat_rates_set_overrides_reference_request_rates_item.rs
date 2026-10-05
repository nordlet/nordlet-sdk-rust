pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuVatRatesSetOverridesReferenceRequestRatesItem {
    pub category: EuVatRatesSetOverridesReferenceRequestRatesItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
}

impl EuVatRatesSetOverridesReferenceRequestRatesItem {
    pub fn builder() -> EuVatRatesSetOverridesReferenceRequestRatesItemBuilder {
        <EuVatRatesSetOverridesReferenceRequestRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesSetOverridesReferenceRequestRatesItemBuilder {
    category: Option<EuVatRatesSetOverridesReferenceRequestRatesItemCategory>,
    rate_percent: Option<String>,
}

impl EuVatRatesSetOverridesReferenceRequestRatesItemBuilder {
    pub fn category(
        mut self,
        value: EuVatRatesSetOverridesReferenceRequestRatesItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesSetOverridesReferenceRequestRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](EuVatRatesSetOverridesReferenceRequestRatesItemBuilder::category)
    /// - [`rate_percent`](EuVatRatesSetOverridesReferenceRequestRatesItemBuilder::rate_percent)
    pub fn build(self) -> Result<EuVatRatesSetOverridesReferenceRequestRatesItem, BuildError> {
        Ok(EuVatRatesSetOverridesReferenceRequestRatesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
        })
    }
}
