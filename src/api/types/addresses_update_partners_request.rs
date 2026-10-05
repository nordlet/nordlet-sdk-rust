pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddressesUpdatePartnersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AddressesUpdatePartnersRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default)]
    pub id: String,
}

impl AddressesUpdatePartnersRequest {
    pub fn builder() -> AddressesUpdatePartnersRequestBuilder {
        <AddressesUpdatePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddressesUpdatePartnersRequestBuilder {
    r#type: Option<AddressesUpdatePartnersRequestType>,
    street: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country_code: Option<String>,
    is_default: Option<bool>,
    id: Option<String>,
}

impl AddressesUpdatePartnersRequestBuilder {
    pub fn r#type(mut self, value: AddressesUpdatePartnersRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddressesUpdatePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AddressesUpdatePartnersRequestBuilder::id)
    pub fn build(self) -> Result<AddressesUpdatePartnersRequest, BuildError> {
        Ok(AddressesUpdatePartnersRequest {
            r#type: self.r#type,
            street: self.street,
            city: self.city,
            postal_code: self.postal_code,
            country_code: self.country_code,
            is_default: self.is_default,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
