pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1CashOrdersGetResponse {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1CashOrdersGetResponseType,
    #[serde(default)]
    pub series: String,
    #[serde(default)]
    pub number: i64,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(default)]
    pub date: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(rename = "cashAccountCode")]
    #[serde(default)]
    pub cash_account_code: String,
    #[serde(rename = "counterAccountCode")]
    #[serde(default)]
    pub counter_account_code: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1CashOrdersGetResponse {
    pub fn builder() -> PostV1CashOrdersGetResponseBuilder {
        <PostV1CashOrdersGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashOrdersGetResponseBuilder {
    id: Option<String>,
    r#type: Option<PostV1CashOrdersGetResponseType>,
    series: Option<String>,
    number: Option<i64>,
    full_number: Option<String>,
    date: Option<String>,
    partner_id: Option<String>,
    employee_id: Option<String>,
    amount: Option<String>,
    currency: Option<String>,
    purpose: Option<String>,
    cash_account_code: Option<String>,
    counter_account_code: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1CashOrdersGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1CashOrdersGetResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
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

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
        self
    }

    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn counter_account_code(mut self, value: impl Into<String>) -> Self {
        self.counter_account_code = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashOrdersGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CashOrdersGetResponseBuilder::id)
    /// - [`r#type`](PostV1CashOrdersGetResponseBuilder::r#type)
    /// - [`series`](PostV1CashOrdersGetResponseBuilder::series)
    /// - [`number`](PostV1CashOrdersGetResponseBuilder::number)
    /// - [`full_number`](PostV1CashOrdersGetResponseBuilder::full_number)
    /// - [`date`](PostV1CashOrdersGetResponseBuilder::date)
    /// - [`amount`](PostV1CashOrdersGetResponseBuilder::amount)
    /// - [`currency`](PostV1CashOrdersGetResponseBuilder::currency)
    /// - [`purpose`](PostV1CashOrdersGetResponseBuilder::purpose)
    /// - [`cash_account_code`](PostV1CashOrdersGetResponseBuilder::cash_account_code)
    /// - [`counter_account_code`](PostV1CashOrdersGetResponseBuilder::counter_account_code)
    /// - [`created_at`](PostV1CashOrdersGetResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1CashOrdersGetResponse, BuildError> {
        Ok(PostV1CashOrdersGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            series: self
                .series
                .ok_or_else(|| BuildError::missing_field("series"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            full_number: self
                .full_number
                .ok_or_else(|| BuildError::missing_field("full_number"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            partner_id: self.partner_id,
            employee_id: self.employee_id,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
            cash_account_code: self
                .cash_account_code
                .ok_or_else(|| BuildError::missing_field("cash_account_code"))?,
            counter_account_code: self
                .counter_account_code
                .ok_or_else(|| BuildError::missing_field("counter_account_code"))?,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
