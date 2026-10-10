pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePlatformSellersRequestTaxResidencesItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
}

impl CreatePlatformSellersRequestTaxResidencesItem {
    pub fn builder() -> CreatePlatformSellersRequestTaxResidencesItemBuilder {
        <CreatePlatformSellersRequestTaxResidencesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformSellersRequestTaxResidencesItemBuilder {
    country_code: Option<String>,
    tin: Option<String>,
}

impl CreatePlatformSellersRequestTaxResidencesItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn tin(mut self, value: impl Into<String>) -> Self {
        self.tin = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformSellersRequestTaxResidencesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](CreatePlatformSellersRequestTaxResidencesItemBuilder::country_code)
    pub fn build(self) -> Result<CreatePlatformSellersRequestTaxResidencesItem, BuildError> {
        Ok(CreatePlatformSellersRequestTaxResidencesItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            tin: self.tin,
        })
    }
}
