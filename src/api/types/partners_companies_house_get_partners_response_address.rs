pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompaniesHouseGetPartnersResponseAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: CompaniesHouseGetPartnersResponseAddressCountryCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision: Option<String>,
}

impl CompaniesHouseGetPartnersResponseAddress {
    pub fn builder() -> CompaniesHouseGetPartnersResponseAddressBuilder {
        <CompaniesHouseGetPartnersResponseAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesHouseGetPartnersResponseAddressBuilder {
    street: Option<String>,
    city: Option<String>,
    county: Option<String>,
    postal_code: Option<String>,
    country_code: Option<CompaniesHouseGetPartnersResponseAddressCountryCode>,
    subdivision: Option<String>,
}

impl CompaniesHouseGetPartnersResponseAddressBuilder {
    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn county(mut self, value: impl Into<String>) -> Self {
        self.county = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn country_code(
        mut self,
        value: CompaniesHouseGetPartnersResponseAddressCountryCode,
    ) -> Self {
        self.country_code = Some(value);
        self
    }

    pub fn subdivision(mut self, value: impl Into<String>) -> Self {
        self.subdivision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesHouseGetPartnersResponseAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](CompaniesHouseGetPartnersResponseAddressBuilder::country_code)
    pub fn build(self) -> Result<CompaniesHouseGetPartnersResponseAddress, BuildError> {
        Ok(CompaniesHouseGetPartnersResponseAddress {
            street: self.street,
            city: self.city,
            county: self.county,
            postal_code: self.postal_code,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            subdivision: self.subdivision,
        })
    }
}
