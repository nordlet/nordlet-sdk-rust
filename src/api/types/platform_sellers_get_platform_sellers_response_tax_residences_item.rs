pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPlatformSellersResponseTaxResidencesItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
}

impl GetPlatformSellersResponseTaxResidencesItem {
    pub fn builder() -> GetPlatformSellersResponseTaxResidencesItemBuilder {
        <GetPlatformSellersResponseTaxResidencesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPlatformSellersResponseTaxResidencesItemBuilder {
    country_code: Option<String>,
    tin: Option<String>,
}

impl GetPlatformSellersResponseTaxResidencesItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn tin(mut self, value: impl Into<String>) -> Self {
        self.tin = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPlatformSellersResponseTaxResidencesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](GetPlatformSellersResponseTaxResidencesItemBuilder::country_code)
    pub fn build(self) -> Result<GetPlatformSellersResponseTaxResidencesItem, BuildError> {
        Ok(GetPlatformSellersResponseTaxResidencesItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            tin: self.tin,
        })
    }
}
