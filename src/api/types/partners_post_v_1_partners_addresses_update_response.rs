pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAddressesUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub r#type: String,
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
    #[serde(default)]
    pub is_default: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PartnersAddressesUpdateResponse {
    pub fn builder() -> PostV1PartnersAddressesUpdateResponseBuilder {
        <PostV1PartnersAddressesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAddressesUpdateResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<String>,
    street: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country_code: Option<String>,
    is_default: Option<bool>,
    created_at: Option<String>,
}

impl PostV1PartnersAddressesUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAddressesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersAddressesUpdateResponseBuilder::id)
    /// - [`partner_id`](PostV1PartnersAddressesUpdateResponseBuilder::partner_id)
    /// - [`r#type`](PostV1PartnersAddressesUpdateResponseBuilder::r#type)
    /// - [`is_default`](PostV1PartnersAddressesUpdateResponseBuilder::is_default)
    /// - [`created_at`](PostV1PartnersAddressesUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1PartnersAddressesUpdateResponse, BuildError> {
        Ok(PostV1PartnersAddressesUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            street: self.street,
            city: self.city,
            postal_code: self.postal_code,
            country_code: self.country_code,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
