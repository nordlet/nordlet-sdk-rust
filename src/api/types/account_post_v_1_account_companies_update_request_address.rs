pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesUpdateRequestAddress {
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
}

impl PostV1AccountCompaniesUpdateRequestAddress {
    pub fn builder() -> PostV1AccountCompaniesUpdateRequestAddressBuilder {
        <PostV1AccountCompaniesUpdateRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesUpdateRequestAddressBuilder {
    street: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country_code: Option<String>,
}

impl PostV1AccountCompaniesUpdateRequestAddressBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesUpdateRequestAddress`].
    pub fn build(self) -> Result<PostV1AccountCompaniesUpdateRequestAddress, BuildError> {
        Ok(PostV1AccountCompaniesUpdateRequestAddress {
            street: self.street,
            city: self.city,
            postal_code: self.postal_code,
            country_code: self.country_code,
        })
    }
}
