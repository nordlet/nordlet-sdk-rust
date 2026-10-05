pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsImportBankRequestTransactionsItem {
    #[serde(default)]
    pub date: NaiveDate,
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

impl TransactionsImportBankRequestTransactionsItem {
    pub fn builder() -> TransactionsImportBankRequestTransactionsItemBuilder {
        <TransactionsImportBankRequestTransactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsImportBankRequestTransactionsItemBuilder {
    date: Option<NaiveDate>,
    amount: Option<String>,
    currency: Option<String>,
    counterparty_name: Option<String>,
    counterparty_iban: Option<String>,
    description: Option<String>,
    external_id: Option<String>,
}

impl TransactionsImportBankRequestTransactionsItemBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    /// Consumes the builder and constructs a [`TransactionsImportBankRequestTransactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](TransactionsImportBankRequestTransactionsItemBuilder::date)
    /// - [`amount`](TransactionsImportBankRequestTransactionsItemBuilder::amount)
    pub fn build(self) -> Result<TransactionsImportBankRequestTransactionsItem, BuildError> {
        Ok(TransactionsImportBankRequestTransactionsItem {
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
