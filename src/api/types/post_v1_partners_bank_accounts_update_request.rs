pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersBankAccountsUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
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
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersBankAccountsUpdateRequest {
    pub fn builder() -> PostV1PartnersBankAccountsUpdateRequestBuilder {
        <PostV1PartnersBankAccountsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersBankAccountsUpdateRequestBuilder {
    iban: Option<String>,
    bank_name: Option<String>,
    bic: Option<String>,
    currency: Option<String>,
    is_default: Option<bool>,
    id: Option<String>,
}

impl PostV1PartnersBankAccountsUpdateRequestBuilder {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersBankAccountsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersBankAccountsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersBankAccountsUpdateRequest, BuildError> {
        Ok(PostV1PartnersBankAccountsUpdateRequest {
            iban: self.iban,
            bank_name: self.bank_name,
            bic: self.bic,
            currency: self.currency,
            is_default: self.is_default,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
