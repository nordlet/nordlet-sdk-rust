pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod {
    pub fn builder() -> GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriodBuilder {
        <GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriodBuilder {
    from: Option<String>,
    to: Option<String>,
}

impl GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriodBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriodBuilder::from)
    /// - [`to`](GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriodBuilder::to)
    pub fn build(
        self,
    ) -> Result<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod, BuildError> {
        Ok(
            GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod {
                from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
                to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            },
        )
    }
}
