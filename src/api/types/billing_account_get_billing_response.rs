pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountGetBillingResponse {
    pub plan: AccountGetBillingResponsePlan,
    pub status: AccountGetBillingResponseStatus,
    #[serde(rename = "balanceCents")]
    #[serde(default)]
    pub balance_cents: i64,
    #[serde(rename = "trialEndsAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub trial_ends_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "firstTopUpAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub first_top_up_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "lastChargedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_charged_date: Option<NaiveDate>,
    #[serde(rename = "paymentsConfigured")]
    #[serde(default)]
    pub payments_configured: bool,
    #[serde(rename = "hasPaymentAccount")]
    #[serde(default)]
    pub has_payment_account: bool,
    #[serde(rename = "hasSubscription")]
    #[serde(default)]
    pub has_subscription: bool,
    #[serde(rename = "paymentFailedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub payment_failed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "paymentFailedInvoiceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_failed_invoice_url: Option<String>,
    #[serde(rename = "monthToDate")]
    #[serde(default)]
    pub month_to_date: AccountGetBillingResponseMonthToDate,
    #[serde(default)]
    pub plans: HashMap<String, AccountGetBillingResponsePlansValue>,
    #[serde(rename = "topUp")]
    #[serde(default)]
    pub top_up: AccountGetBillingResponseTopUp,
    #[serde(rename = "trialDays")]
    #[serde(default)]
    pub trial_days: i64,
}

impl AccountGetBillingResponse {
    pub fn builder() -> AccountGetBillingResponseBuilder {
        <AccountGetBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountGetBillingResponseBuilder {
    plan: Option<AccountGetBillingResponsePlan>,
    status: Option<AccountGetBillingResponseStatus>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<DateTime<FixedOffset>>,
    first_top_up_at: Option<DateTime<FixedOffset>>,
    last_charged_date: Option<NaiveDate>,
    payments_configured: Option<bool>,
    has_payment_account: Option<bool>,
    has_subscription: Option<bool>,
    payment_failed_at: Option<DateTime<FixedOffset>>,
    payment_failed_invoice_url: Option<String>,
    month_to_date: Option<AccountGetBillingResponseMonthToDate>,
    plans: Option<HashMap<String, AccountGetBillingResponsePlansValue>>,
    top_up: Option<AccountGetBillingResponseTopUp>,
    trial_days: Option<i64>,
}

impl AccountGetBillingResponseBuilder {
    pub fn plan(mut self, value: AccountGetBillingResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn status(mut self, value: AccountGetBillingResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn balance_cents(mut self, value: i64) -> Self {
        self.balance_cents = Some(value);
        self
    }

    pub fn trial_ends_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.trial_ends_at = Some(value);
        self
    }

    pub fn first_top_up_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_top_up_at = Some(value);
        self
    }

    pub fn last_charged_date(mut self, value: NaiveDate) -> Self {
        self.last_charged_date = Some(value);
        self
    }

    pub fn payments_configured(mut self, value: bool) -> Self {
        self.payments_configured = Some(value);
        self
    }

    pub fn has_payment_account(mut self, value: bool) -> Self {
        self.has_payment_account = Some(value);
        self
    }

    pub fn has_subscription(mut self, value: bool) -> Self {
        self.has_subscription = Some(value);
        self
    }

    pub fn payment_failed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.payment_failed_at = Some(value);
        self
    }

    pub fn payment_failed_invoice_url(mut self, value: impl Into<String>) -> Self {
        self.payment_failed_invoice_url = Some(value.into());
        self
    }

    pub fn month_to_date(mut self, value: AccountGetBillingResponseMonthToDate) -> Self {
        self.month_to_date = Some(value);
        self
    }

    pub fn plans(mut self, value: HashMap<String, AccountGetBillingResponsePlansValue>) -> Self {
        self.plans = Some(value);
        self
    }

    pub fn top_up(mut self, value: AccountGetBillingResponseTopUp) -> Self {
        self.top_up = Some(value);
        self
    }

    pub fn trial_days(mut self, value: i64) -> Self {
        self.trial_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountGetBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](AccountGetBillingResponseBuilder::plan)
    /// - [`status`](AccountGetBillingResponseBuilder::status)
    /// - [`balance_cents`](AccountGetBillingResponseBuilder::balance_cents)
    /// - [`payments_configured`](AccountGetBillingResponseBuilder::payments_configured)
    /// - [`has_payment_account`](AccountGetBillingResponseBuilder::has_payment_account)
    /// - [`has_subscription`](AccountGetBillingResponseBuilder::has_subscription)
    /// - [`month_to_date`](AccountGetBillingResponseBuilder::month_to_date)
    /// - [`plans`](AccountGetBillingResponseBuilder::plans)
    /// - [`top_up`](AccountGetBillingResponseBuilder::top_up)
    /// - [`trial_days`](AccountGetBillingResponseBuilder::trial_days)
    pub fn build(self) -> Result<AccountGetBillingResponse, BuildError> {
        Ok(AccountGetBillingResponse {
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            balance_cents: self
                .balance_cents
                .ok_or_else(|| BuildError::missing_field("balance_cents"))?,
            trial_ends_at: self.trial_ends_at,
            first_top_up_at: self.first_top_up_at,
            last_charged_date: self.last_charged_date,
            payments_configured: self
                .payments_configured
                .ok_or_else(|| BuildError::missing_field("payments_configured"))?,
            has_payment_account: self
                .has_payment_account
                .ok_or_else(|| BuildError::missing_field("has_payment_account"))?,
            has_subscription: self
                .has_subscription
                .ok_or_else(|| BuildError::missing_field("has_subscription"))?,
            payment_failed_at: self.payment_failed_at,
            payment_failed_invoice_url: self.payment_failed_invoice_url,
            month_to_date: self
                .month_to_date
                .ok_or_else(|| BuildError::missing_field("month_to_date"))?,
            plans: self
                .plans
                .ok_or_else(|| BuildError::missing_field("plans"))?,
            top_up: self
                .top_up
                .ok_or_else(|| BuildError::missing_field("top_up"))?,
            trial_days: self
                .trial_days
                .ok_or_else(|| BuildError::missing_field("trial_days"))?,
        })
    }
}
