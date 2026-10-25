pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlatRateConfirmBillingResponse {
    pub plan: FlatRateConfirmBillingResponsePlan,
    #[serde(rename = "accountType")]
    pub account_type: FlatRateConfirmBillingResponseAccountType,
    pub status: FlatRateConfirmBillingResponseStatus,
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
    /// When the Stripe subscription ends after `subscription/cancel`; null while it renews
    #[serde(rename = "subscriptionCancelAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub subscription_cancel_at: Option<DateTime<FixedOffset>>,
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
    pub month_to_date: FlatRateConfirmBillingResponseMonthToDate,
    #[serde(default)]
    pub plans: HashMap<String, FlatRateConfirmBillingResponsePlansValue>,
    #[serde(rename = "apiRequestEur")]
    #[serde(default)]
    pub api_request_eur: String,
    #[serde(rename = "topUp")]
    #[serde(default)]
    pub top_up: FlatRateConfirmBillingResponseTopUp,
    #[serde(rename = "autoTopUp")]
    #[serde(default)]
    pub auto_top_up: FlatRateConfirmBillingResponseAutoTopUp,
    #[serde(rename = "overdraftCapCents")]
    #[serde(default)]
    pub overdraft_cap_cents: i64,
    #[serde(rename = "overdraftCapMaxCents")]
    #[serde(default)]
    pub overdraft_cap_max_cents: i64,
    #[serde(rename = "trialDays")]
    #[serde(default)]
    pub trial_days: i64,
}

impl FlatRateConfirmBillingResponse {
    pub fn builder() -> FlatRateConfirmBillingResponseBuilder {
        <FlatRateConfirmBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlatRateConfirmBillingResponseBuilder {
    plan: Option<FlatRateConfirmBillingResponsePlan>,
    account_type: Option<FlatRateConfirmBillingResponseAccountType>,
    status: Option<FlatRateConfirmBillingResponseStatus>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<DateTime<FixedOffset>>,
    first_top_up_at: Option<DateTime<FixedOffset>>,
    last_charged_date: Option<NaiveDate>,
    payments_configured: Option<bool>,
    has_payment_account: Option<bool>,
    has_subscription: Option<bool>,
    subscription_cancel_at: Option<DateTime<FixedOffset>>,
    payment_failed_at: Option<DateTime<FixedOffset>>,
    payment_failed_invoice_url: Option<String>,
    month_to_date: Option<FlatRateConfirmBillingResponseMonthToDate>,
    plans: Option<HashMap<String, FlatRateConfirmBillingResponsePlansValue>>,
    api_request_eur: Option<String>,
    top_up: Option<FlatRateConfirmBillingResponseTopUp>,
    auto_top_up: Option<FlatRateConfirmBillingResponseAutoTopUp>,
    overdraft_cap_cents: Option<i64>,
    overdraft_cap_max_cents: Option<i64>,
    trial_days: Option<i64>,
}

impl FlatRateConfirmBillingResponseBuilder {
    pub fn plan(mut self, value: FlatRateConfirmBillingResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn account_type(mut self, value: FlatRateConfirmBillingResponseAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn status(mut self, value: FlatRateConfirmBillingResponseStatus) -> Self {
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

    pub fn subscription_cancel_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.subscription_cancel_at = Some(value);
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

    pub fn month_to_date(mut self, value: FlatRateConfirmBillingResponseMonthToDate) -> Self {
        self.month_to_date = Some(value);
        self
    }

    pub fn plans(
        mut self,
        value: HashMap<String, FlatRateConfirmBillingResponsePlansValue>,
    ) -> Self {
        self.plans = Some(value);
        self
    }

    pub fn api_request_eur(mut self, value: impl Into<String>) -> Self {
        self.api_request_eur = Some(value.into());
        self
    }

    pub fn top_up(mut self, value: FlatRateConfirmBillingResponseTopUp) -> Self {
        self.top_up = Some(value);
        self
    }

    pub fn auto_top_up(mut self, value: FlatRateConfirmBillingResponseAutoTopUp) -> Self {
        self.auto_top_up = Some(value);
        self
    }

    pub fn overdraft_cap_cents(mut self, value: i64) -> Self {
        self.overdraft_cap_cents = Some(value);
        self
    }

    pub fn overdraft_cap_max_cents(mut self, value: i64) -> Self {
        self.overdraft_cap_max_cents = Some(value);
        self
    }

    pub fn trial_days(mut self, value: i64) -> Self {
        self.trial_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FlatRateConfirmBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](FlatRateConfirmBillingResponseBuilder::plan)
    /// - [`account_type`](FlatRateConfirmBillingResponseBuilder::account_type)
    /// - [`status`](FlatRateConfirmBillingResponseBuilder::status)
    /// - [`balance_cents`](FlatRateConfirmBillingResponseBuilder::balance_cents)
    /// - [`payments_configured`](FlatRateConfirmBillingResponseBuilder::payments_configured)
    /// - [`has_payment_account`](FlatRateConfirmBillingResponseBuilder::has_payment_account)
    /// - [`has_subscription`](FlatRateConfirmBillingResponseBuilder::has_subscription)
    /// - [`month_to_date`](FlatRateConfirmBillingResponseBuilder::month_to_date)
    /// - [`plans`](FlatRateConfirmBillingResponseBuilder::plans)
    /// - [`api_request_eur`](FlatRateConfirmBillingResponseBuilder::api_request_eur)
    /// - [`top_up`](FlatRateConfirmBillingResponseBuilder::top_up)
    /// - [`auto_top_up`](FlatRateConfirmBillingResponseBuilder::auto_top_up)
    /// - [`overdraft_cap_cents`](FlatRateConfirmBillingResponseBuilder::overdraft_cap_cents)
    /// - [`overdraft_cap_max_cents`](FlatRateConfirmBillingResponseBuilder::overdraft_cap_max_cents)
    /// - [`trial_days`](FlatRateConfirmBillingResponseBuilder::trial_days)
    pub fn build(self) -> Result<FlatRateConfirmBillingResponse, BuildError> {
        Ok(FlatRateConfirmBillingResponse {
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
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
            subscription_cancel_at: self.subscription_cancel_at,
            payment_failed_at: self.payment_failed_at,
            payment_failed_invoice_url: self.payment_failed_invoice_url,
            month_to_date: self
                .month_to_date
                .ok_or_else(|| BuildError::missing_field("month_to_date"))?,
            plans: self
                .plans
                .ok_or_else(|| BuildError::missing_field("plans"))?,
            api_request_eur: self
                .api_request_eur
                .ok_or_else(|| BuildError::missing_field("api_request_eur"))?,
            top_up: self
                .top_up
                .ok_or_else(|| BuildError::missing_field("top_up"))?,
            auto_top_up: self
                .auto_top_up
                .ok_or_else(|| BuildError::missing_field("auto_top_up"))?,
            overdraft_cap_cents: self
                .overdraft_cap_cents
                .ok_or_else(|| BuildError::missing_field("overdraft_cap_cents"))?,
            overdraft_cap_max_cents: self
                .overdraft_cap_max_cents
                .ok_or_else(|| BuildError::missing_field("overdraft_cap_max_cents"))?,
            trial_days: self
                .trial_days
                .ok_or_else(|| BuildError::missing_field("trial_days"))?,
        })
    }
}
