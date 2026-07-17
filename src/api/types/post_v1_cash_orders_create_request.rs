pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1CashOrdersCreateRequest {
    pub r#type: PostV1CashOrdersCreateRequestType,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(rename = "counterAccountCode")]
    #[serde(default)]
    pub counter_account_code: String,
    #[serde(rename = "cashAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1CashOrdersCreateRequest {
    pub fn builder() -> PostV1CashOrdersCreateRequestBuilder {
        <PostV1CashOrdersCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashOrdersCreateRequestBuilder {
    r#type: Option<PostV1CashOrdersCreateRequestType>,
    date: Option<String>,
    amount: Option<String>,
    purpose: Option<String>,
    counter_account_code: Option<String>,
    cash_account_code: Option<String>,
    series: Option<String>,
    partner_id: Option<String>,
    employee_id: Option<String>,
    notes: Option<String>,
}

impl PostV1CashOrdersCreateRequestBuilder {
    pub fn r#type(mut self, value: PostV1CashOrdersCreateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
        self
    }

    pub fn counter_account_code(mut self, value: impl Into<String>) -> Self {
        self.counter_account_code = Some(value.into());
        self
    }

    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashOrdersCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostV1CashOrdersCreateRequestBuilder::r#type)
    /// - [`date`](PostV1CashOrdersCreateRequestBuilder::date)
    /// - [`amount`](PostV1CashOrdersCreateRequestBuilder::amount)
    /// - [`purpose`](PostV1CashOrdersCreateRequestBuilder::purpose)
    /// - [`counter_account_code`](PostV1CashOrdersCreateRequestBuilder::counter_account_code)
    pub fn build(self) -> Result<PostV1CashOrdersCreateRequest, BuildError> {
        Ok(PostV1CashOrdersCreateRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
            counter_account_code: self
                .counter_account_code
                .ok_or_else(|| BuildError::missing_field("counter_account_code"))?,
            cash_account_code: self.cash_account_code,
            series: self.series,
            partner_id: self.partner_id,
            employee_id: self.employee_id,
            notes: self.notes,
        })
    }
}
