pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsImportRequestTransactionsItem {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "counterpartyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_name: Option<String>,
    #[serde(rename = "counterpartyIban")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl PostV1BankTransactionsImportRequestTransactionsItem {
    pub fn builder() -> PostV1BankTransactionsImportRequestTransactionsItemBuilder {
        <PostV1BankTransactionsImportRequestTransactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsImportRequestTransactionsItemBuilder {
    date: Option<String>,
    amount: Option<String>,
    currency: Option<String>,
    counterparty_name: Option<String>,
    counterparty_iban: Option<String>,
    description: Option<String>,
    external_id: Option<String>,
}

impl PostV1BankTransactionsImportRequestTransactionsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn counterparty_name(mut self, value: impl Into<String>) -> Self {
        self.counterparty_name = Some(value.into());
        self
    }

    pub fn counterparty_iban(mut self, value: impl Into<String>) -> Self {
        self.counterparty_iban = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsImportRequestTransactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1BankTransactionsImportRequestTransactionsItemBuilder::date)
    /// - [`amount`](PostV1BankTransactionsImportRequestTransactionsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1BankTransactionsImportRequestTransactionsItem, BuildError> {
        Ok(PostV1BankTransactionsImportRequestTransactionsItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self.currency,
            counterparty_name: self.counterparty_name,
            counterparty_iban: self.counterparty_iban,
            description: self.description,
            external_id: self.external_id,
        })
    }
}
