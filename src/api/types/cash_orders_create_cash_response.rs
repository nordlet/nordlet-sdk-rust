pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrdersCreateCashResponse {
    #[serde(default)]
    pub id: String,
    pub r#type: OrdersCreateCashResponseType,
    #[serde(default)]
    pub series: String,
    #[serde(default)]
    pub number: i64,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(default)]
    pub date: NaiveDate,
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
    #[serde(rename = "saleInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_invoice_id: Option<String>,
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl OrdersCreateCashResponse {
    pub fn builder() -> OrdersCreateCashResponseBuilder {
        <OrdersCreateCashResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCreateCashResponseBuilder {
    id: Option<String>,
    r#type: Option<OrdersCreateCashResponseType>,
    series: Option<String>,
    number: Option<i64>,
    full_number: Option<String>,
    date: Option<NaiveDate>,
    partner_id: Option<String>,
    employee_id: Option<String>,
    amount: Option<String>,
    currency: Option<String>,
    purpose: Option<String>,
    cash_account_code: Option<String>,
    counter_account_code: Option<String>,
    journal_transaction_id: Option<String>,
    sale_invoice_id: Option<String>,
    purchase_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl OrdersCreateCashResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: OrdersCreateCashResponseType) -> Self {
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

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    pub fn sale_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.sale_invoice_id = Some(value.into());
        self
    }

    pub fn purchase_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_invoice_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersCreateCashResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersCreateCashResponseBuilder::id)
    /// - [`r#type`](OrdersCreateCashResponseBuilder::r#type)
    /// - [`series`](OrdersCreateCashResponseBuilder::series)
    /// - [`number`](OrdersCreateCashResponseBuilder::number)
    /// - [`full_number`](OrdersCreateCashResponseBuilder::full_number)
    /// - [`date`](OrdersCreateCashResponseBuilder::date)
    /// - [`amount`](OrdersCreateCashResponseBuilder::amount)
    /// - [`currency`](OrdersCreateCashResponseBuilder::currency)
    /// - [`purpose`](OrdersCreateCashResponseBuilder::purpose)
    /// - [`cash_account_code`](OrdersCreateCashResponseBuilder::cash_account_code)
    /// - [`counter_account_code`](OrdersCreateCashResponseBuilder::counter_account_code)
    /// - [`created_at`](OrdersCreateCashResponseBuilder::created_at)
    pub fn build(self) -> Result<OrdersCreateCashResponse, BuildError> {
        Ok(OrdersCreateCashResponse {
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
            sale_invoice_id: self.sale_invoice_id,
            purchase_invoice_id: self.purchase_invoice_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
