pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersBankAccountsCreateRequest {
    #[serde(default)]
    pub iban: String,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl PostV1PartnersBankAccountsCreateRequest {
    pub fn builder() -> PostV1PartnersBankAccountsCreateRequestBuilder {
        <PostV1PartnersBankAccountsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersBankAccountsCreateRequestBuilder {
    iban: Option<String>,
    bank_name: Option<String>,
    bic: Option<String>,
    currency: Option<String>,
    is_default: Option<bool>,
    partner_id: Option<String>,
}

impl PostV1PartnersBankAccountsCreateRequestBuilder {
    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersBankAccountsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`iban`](PostV1PartnersBankAccountsCreateRequestBuilder::iban)
    /// - [`partner_id`](PostV1PartnersBankAccountsCreateRequestBuilder::partner_id)
    pub fn build(self) -> Result<PostV1PartnersBankAccountsCreateRequest, BuildError> {
        Ok(PostV1PartnersBankAccountsCreateRequest {
            iban: self.iban.ok_or_else(|| BuildError::missing_field("iban"))?,
            bank_name: self.bank_name,
            bic: self.bic,
            currency: self.currency,
            is_default: self.is_default,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
