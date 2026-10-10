pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPlatformSellersResponseRowsItemAddress {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "buildingIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_identifier: Option<String>,
    #[serde(rename = "postCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<String>,
}

impl ListPlatformSellersResponseRowsItemAddress {
    pub fn builder() -> ListPlatformSellersResponseRowsItemAddressBuilder {
        <ListPlatformSellersResponseRowsItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPlatformSellersResponseRowsItemAddressBuilder {
    country_code: Option<String>,
    street: Option<String>,
    building_identifier: Option<String>,
    post_code: Option<String>,
    city: Option<String>,
    free: Option<String>,
}

impl ListPlatformSellersResponseRowsItemAddressBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn building_identifier(mut self, value: impl Into<String>) -> Self {
        self.building_identifier = Some(value.into());
        self
    }

    pub fn post_code(mut self, value: impl Into<String>) -> Self {
        self.post_code = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn free(mut self, value: impl Into<String>) -> Self {
        self.free = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPlatformSellersResponseRowsItemAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](ListPlatformSellersResponseRowsItemAddressBuilder::country_code)
    pub fn build(self) -> Result<ListPlatformSellersResponseRowsItemAddress, BuildError> {
        Ok(ListPlatformSellersResponseRowsItemAddress {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            street: self.street,
            building_identifier: self.building_identifier,
            post_code: self.post_code,
            city: self.city,
            free: self.free,
        })
    }
}
