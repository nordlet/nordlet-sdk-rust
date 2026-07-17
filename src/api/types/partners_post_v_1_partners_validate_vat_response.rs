pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersValidateVatResponse {
    #[serde(default)]
    pub valid: bool,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "vatNumber")]
    #[serde(default)]
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "requestIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
    #[serde(rename = "checkedAt")]
    #[serde(default)]
    pub checked_at: String,
}

impl PostV1PartnersValidateVatResponse {
    pub fn builder() -> PostV1PartnersValidateVatResponseBuilder {
        <PostV1PartnersValidateVatResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersValidateVatResponseBuilder {
    valid: Option<bool>,
    country_code: Option<String>,
    vat_number: Option<String>,
    name: Option<String>,
    address: Option<String>,
    request_identifier: Option<String>,
    checked_at: Option<String>,
}

impl PostV1PartnersValidateVatResponseBuilder {
    pub fn valid(mut self, value: bool) -> Self {
        self.valid = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn vat_number(mut self, value: impl Into<String>) -> Self {
        self.vat_number = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn request_identifier(mut self, value: impl Into<String>) -> Self {
        self.request_identifier = Some(value.into());
        self
    }

    pub fn checked_at(mut self, value: impl Into<String>) -> Self {
        self.checked_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersValidateVatResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`valid`](PostV1PartnersValidateVatResponseBuilder::valid)
    /// - [`country_code`](PostV1PartnersValidateVatResponseBuilder::country_code)
    /// - [`vat_number`](PostV1PartnersValidateVatResponseBuilder::vat_number)
    /// - [`checked_at`](PostV1PartnersValidateVatResponseBuilder::checked_at)
    pub fn build(self) -> Result<PostV1PartnersValidateVatResponse, BuildError> {
        Ok(PostV1PartnersValidateVatResponse {
            valid: self
                .valid
                .ok_or_else(|| BuildError::missing_field("valid"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            vat_number: self
                .vat_number
                .ok_or_else(|| BuildError::missing_field("vat_number"))?,
            name: self.name,
            address: self.address,
            request_identifier: self.request_identifier,
            checked_at: self
                .checked_at
                .ok_or_else(|| BuildError::missing_field("checked_at"))?,
        })
    }
}
