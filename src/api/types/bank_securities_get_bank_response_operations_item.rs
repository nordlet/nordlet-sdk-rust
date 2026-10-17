pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SecuritiesGetBankResponseOperationsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "assetCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "brokerRealized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_realized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "fxRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_rate: Option<String>,
    #[serde(rename = "cashBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_base: Option<String>,
    #[serde(rename = "costBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_base: Option<String>,
    #[serde(rename = "realizedBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realized_base: Option<String>,
    pub status: SecuritiesGetBankResponseOperationsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
}

impl SecuritiesGetBankResponseOperationsItem {
    pub fn builder() -> SecuritiesGetBankResponseOperationsItemBuilder {
        <SecuritiesGetBankResponseOperationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesGetBankResponseOperationsItemBuilder {
    id: Option<String>,
    bank_account_id: Option<String>,
    kind: Option<String>,
    date: Option<NaiveDate>,
    time: Option<String>,
    asset_category: Option<String>,
    symbol: Option<String>,
    description: Option<String>,
    currency: Option<String>,
    quantity: Option<String>,
    price: Option<String>,
    amount: Option<String>,
    fee: Option<String>,
    broker_realized: Option<String>,
    code: Option<String>,
    fx_rate: Option<String>,
    cash_base: Option<String>,
    cost_base: Option<String>,
    realized_base: Option<String>,
    status: Option<SecuritiesGetBankResponseOperationsItemStatus>,
    note: Option<String>,
    journal_transaction_id: Option<String>,
}

impl SecuritiesGetBankResponseOperationsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn time(mut self, value: impl Into<String>) -> Self {
        self.time = Some(value.into());
        self
    }

    pub fn asset_category(mut self, value: impl Into<String>) -> Self {
        self.asset_category = Some(value.into());
        self
    }

    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn price(mut self, value: impl Into<String>) -> Self {
        self.price = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn fee(mut self, value: impl Into<String>) -> Self {
        self.fee = Some(value.into());
        self
    }

    pub fn broker_realized(mut self, value: impl Into<String>) -> Self {
        self.broker_realized = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn fx_rate(mut self, value: impl Into<String>) -> Self {
        self.fx_rate = Some(value.into());
        self
    }

    pub fn cash_base(mut self, value: impl Into<String>) -> Self {
        self.cash_base = Some(value.into());
        self
    }

    pub fn cost_base(mut self, value: impl Into<String>) -> Self {
        self.cost_base = Some(value.into());
        self
    }

    pub fn realized_base(mut self, value: impl Into<String>) -> Self {
        self.realized_base = Some(value.into());
        self
    }

    pub fn status(mut self, value: SecuritiesGetBankResponseOperationsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesGetBankResponseOperationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SecuritiesGetBankResponseOperationsItemBuilder::id)
    /// - [`bank_account_id`](SecuritiesGetBankResponseOperationsItemBuilder::bank_account_id)
    /// - [`kind`](SecuritiesGetBankResponseOperationsItemBuilder::kind)
    /// - [`date`](SecuritiesGetBankResponseOperationsItemBuilder::date)
    /// - [`currency`](SecuritiesGetBankResponseOperationsItemBuilder::currency)
    /// - [`status`](SecuritiesGetBankResponseOperationsItemBuilder::status)
    pub fn build(self) -> Result<SecuritiesGetBankResponseOperationsItem, BuildError> {
        Ok(SecuritiesGetBankResponseOperationsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            time: self.time,
            asset_category: self.asset_category,
            symbol: self.symbol,
            description: self.description,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            quantity: self.quantity,
            price: self.price,
            amount: self.amount,
            fee: self.fee,
            broker_realized: self.broker_realized,
            code: self.code,
            fx_rate: self.fx_rate,
            cash_base: self.cash_base,
            cost_base: self.cost_base,
            realized_base: self.realized_base,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            note: self.note,
            journal_transaction_id: self.journal_transaction_id,
        })
    }
}
