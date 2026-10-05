pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountsListPartnersResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub iban: String,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl BankAccountsListPartnersResponseRowsItem {
    pub fn builder() -> BankAccountsListPartnersResponseRowsItemBuilder {
        <BankAccountsListPartnersResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsListPartnersResponseRowsItemBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    bic: Option<String>,
    currency: Option<String>,
    is_default: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl BankAccountsListPartnersResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsListPartnersResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BankAccountsListPartnersResponseRowsItemBuilder::id)
    /// - [`partner_id`](BankAccountsListPartnersResponseRowsItemBuilder::partner_id)
    /// - [`iban`](BankAccountsListPartnersResponseRowsItemBuilder::iban)
    /// - [`currency`](BankAccountsListPartnersResponseRowsItemBuilder::currency)
    /// - [`is_default`](BankAccountsListPartnersResponseRowsItemBuilder::is_default)
    /// - [`created_at`](BankAccountsListPartnersResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<BankAccountsListPartnersResponseRowsItem, BuildError> {
        Ok(BankAccountsListPartnersResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            iban: self.iban.ok_or_else(|| BuildError::missing_field("iban"))?,
            bank_name: self.bank_name,
            bic: self.bic,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
