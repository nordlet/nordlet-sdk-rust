pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountSetOverdraftCapBillingResponse {
    pub plan: AccountSetOverdraftCapBillingResponsePlan,
    #[serde(rename = "accountType")]
    pub account_type: AccountSetOverdraftCapBillingResponseAccountType,
    pub status: AccountSetOverdraftCapBillingResponseStatus,
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
    pub month_to_date: AccountSetOverdraftCapBillingResponseMonthToDate,
    #[serde(default)]
    pub plans: HashMap<String, AccountSetOverdraftCapBillingResponsePlansValue>,
    #[serde(rename = "apiRequestEur")]
    #[serde(default)]
    pub api_request_eur: String,
    #[serde(rename = "topUp")]
    #[serde(default)]
    pub top_up: AccountSetOverdraftCapBillingResponseTopUp,
    #[serde(rename = "autoTopUp")]
    #[serde(default)]
    pub auto_top_up: AccountSetOverdraftCapBillingResponseAutoTopUp,
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

impl AccountSetOverdraftCapBillingResponse {
    pub fn builder() -> AccountSetOverdraftCapBillingResponseBuilder {
        <AccountSetOverdraftCapBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetOverdraftCapBillingResponseBuilder {
    plan: Option<AccountSetOverdraftCapBillingResponsePlan>,
    account_type: Option<AccountSetOverdraftCapBillingResponseAccountType>,
    status: Option<AccountSetOverdraftCapBillingResponseStatus>,
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
    month_to_date: Option<AccountSetOverdraftCapBillingResponseMonthToDate>,
    plans: Option<HashMap<String, AccountSetOverdraftCapBillingResponsePlansValue>>,
    api_request_eur: Option<String>,
    top_up: Option<AccountSetOverdraftCapBillingResponseTopUp>,
    auto_top_up: Option<AccountSetOverdraftCapBillingResponseAutoTopUp>,
    overdraft_cap_cents: Option<i64>,
    overdraft_cap_max_cents: Option<i64>,
    trial_days: Option<i64>,
}

impl AccountSetOverdraftCapBillingResponseBuilder {
    pub fn plan(mut self, value: AccountSetOverdraftCapBillingResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn account_type(mut self, value: AccountSetOverdraftCapBillingResponseAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn status(mut self, value: AccountSetOverdraftCapBillingResponseStatus) -> Self {
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

    pub fn month_to_date(
        mut self,
        value: AccountSetOverdraftCapBillingResponseMonthToDate,
    ) -> Self {
        self.month_to_date = Some(value);
        self
    }

    pub fn plans(
        mut self,
        value: HashMap<String, AccountSetOverdraftCapBillingResponsePlansValue>,
    ) -> Self {
        self.plans = Some(value);
        self
    }

    pub fn api_request_eur(mut self, value: impl Into<String>) -> Self {
        self.api_request_eur = Some(value.into());
        self
    }

    pub fn top_up(mut self, value: AccountSetOverdraftCapBillingResponseTopUp) -> Self {
        self.top_up = Some(value);
        self
    }

    pub fn auto_top_up(mut self, value: AccountSetOverdraftCapBillingResponseAutoTopUp) -> Self {
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

    /// Consumes the builder and constructs a [`AccountSetOverdraftCapBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](AccountSetOverdraftCapBillingResponseBuilder::plan)
    /// - [`account_type`](AccountSetOverdraftCapBillingResponseBuilder::account_type)
    /// - [`status`](AccountSetOverdraftCapBillingResponseBuilder::status)
    /// - [`balance_cents`](AccountSetOverdraftCapBillingResponseBuilder::balance_cents)
    /// - [`payments_configured`](AccountSetOverdraftCapBillingResponseBuilder::payments_configured)
    /// - [`has_payment_account`](AccountSetOverdraftCapBillingResponseBuilder::has_payment_account)
    /// - [`has_subscription`](AccountSetOverdraftCapBillingResponseBuilder::has_subscription)
    /// - [`month_to_date`](AccountSetOverdraftCapBillingResponseBuilder::month_to_date)
    /// - [`plans`](AccountSetOverdraftCapBillingResponseBuilder::plans)
    /// - [`api_request_eur`](AccountSetOverdraftCapBillingResponseBuilder::api_request_eur)
    /// - [`top_up`](AccountSetOverdraftCapBillingResponseBuilder::top_up)
    /// - [`auto_top_up`](AccountSetOverdraftCapBillingResponseBuilder::auto_top_up)
    /// - [`overdraft_cap_cents`](AccountSetOverdraftCapBillingResponseBuilder::overdraft_cap_cents)
    /// - [`overdraft_cap_max_cents`](AccountSetOverdraftCapBillingResponseBuilder::overdraft_cap_max_cents)
    /// - [`trial_days`](AccountSetOverdraftCapBillingResponseBuilder::trial_days)
    pub fn build(self) -> Result<AccountSetOverdraftCapBillingResponse, BuildError> {
        Ok(AccountSetOverdraftCapBillingResponse {
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
