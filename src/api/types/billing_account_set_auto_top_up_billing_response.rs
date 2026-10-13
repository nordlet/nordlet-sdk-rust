pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountSetAutoTopUpBillingResponse {
    pub plan: AccountSetAutoTopUpBillingResponsePlan,
    #[serde(rename = "accountType")]
    pub account_type: AccountSetAutoTopUpBillingResponseAccountType,
    pub status: AccountSetAutoTopUpBillingResponseStatus,
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
    pub month_to_date: AccountSetAutoTopUpBillingResponseMonthToDate,
    #[serde(default)]
    pub plans: HashMap<String, AccountSetAutoTopUpBillingResponsePlansValue>,
    #[serde(rename = "apiRequestEur")]
    #[serde(default)]
    pub api_request_eur: String,
    #[serde(rename = "topUp")]
    #[serde(default)]
    pub top_up: AccountSetAutoTopUpBillingResponseTopUp,
    #[serde(rename = "autoTopUp")]
    #[serde(default)]
    pub auto_top_up: AccountSetAutoTopUpBillingResponseAutoTopUp,
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

impl AccountSetAutoTopUpBillingResponse {
    pub fn builder() -> AccountSetAutoTopUpBillingResponseBuilder {
        <AccountSetAutoTopUpBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetAutoTopUpBillingResponseBuilder {
    plan: Option<AccountSetAutoTopUpBillingResponsePlan>,
    account_type: Option<AccountSetAutoTopUpBillingResponseAccountType>,
    status: Option<AccountSetAutoTopUpBillingResponseStatus>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<DateTime<FixedOffset>>,
    first_top_up_at: Option<DateTime<FixedOffset>>,
    last_charged_date: Option<NaiveDate>,
    payments_configured: Option<bool>,
    has_payment_account: Option<bool>,
    has_subscription: Option<bool>,
    payment_failed_at: Option<DateTime<FixedOffset>>,
    payment_failed_invoice_url: Option<String>,
    month_to_date: Option<AccountSetAutoTopUpBillingResponseMonthToDate>,
    plans: Option<HashMap<String, AccountSetAutoTopUpBillingResponsePlansValue>>,
    api_request_eur: Option<String>,
    top_up: Option<AccountSetAutoTopUpBillingResponseTopUp>,
    auto_top_up: Option<AccountSetAutoTopUpBillingResponseAutoTopUp>,
    overdraft_cap_cents: Option<i64>,
    overdraft_cap_max_cents: Option<i64>,
    trial_days: Option<i64>,
}

impl AccountSetAutoTopUpBillingResponseBuilder {
    pub fn plan(mut self, value: AccountSetAutoTopUpBillingResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn account_type(mut self, value: AccountSetAutoTopUpBillingResponseAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn status(mut self, value: AccountSetAutoTopUpBillingResponseStatus) -> Self {
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

    pub fn month_to_date(mut self, value: AccountSetAutoTopUpBillingResponseMonthToDate) -> Self {
        self.month_to_date = Some(value);
        self
    }

    pub fn plans(
        mut self,
        value: HashMap<String, AccountSetAutoTopUpBillingResponsePlansValue>,
    ) -> Self {
        self.plans = Some(value);
        self
    }

    pub fn api_request_eur(mut self, value: impl Into<String>) -> Self {
        self.api_request_eur = Some(value.into());
        self
    }

    pub fn top_up(mut self, value: AccountSetAutoTopUpBillingResponseTopUp) -> Self {
        self.top_up = Some(value);
        self
    }

    pub fn auto_top_up(mut self, value: AccountSetAutoTopUpBillingResponseAutoTopUp) -> Self {
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

    /// Consumes the builder and constructs a [`AccountSetAutoTopUpBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](AccountSetAutoTopUpBillingResponseBuilder::plan)
    /// - [`account_type`](AccountSetAutoTopUpBillingResponseBuilder::account_type)
    /// - [`status`](AccountSetAutoTopUpBillingResponseBuilder::status)
    /// - [`balance_cents`](AccountSetAutoTopUpBillingResponseBuilder::balance_cents)
    /// - [`payments_configured`](AccountSetAutoTopUpBillingResponseBuilder::payments_configured)
    /// - [`has_payment_account`](AccountSetAutoTopUpBillingResponseBuilder::has_payment_account)
    /// - [`has_subscription`](AccountSetAutoTopUpBillingResponseBuilder::has_subscription)
    /// - [`month_to_date`](AccountSetAutoTopUpBillingResponseBuilder::month_to_date)
    /// - [`plans`](AccountSetAutoTopUpBillingResponseBuilder::plans)
    /// - [`api_request_eur`](AccountSetAutoTopUpBillingResponseBuilder::api_request_eur)
    /// - [`top_up`](AccountSetAutoTopUpBillingResponseBuilder::top_up)
    /// - [`auto_top_up`](AccountSetAutoTopUpBillingResponseBuilder::auto_top_up)
    /// - [`overdraft_cap_cents`](AccountSetAutoTopUpBillingResponseBuilder::overdraft_cap_cents)
    /// - [`overdraft_cap_max_cents`](AccountSetAutoTopUpBillingResponseBuilder::overdraft_cap_max_cents)
    /// - [`trial_days`](AccountSetAutoTopUpBillingResponseBuilder::trial_days)
    pub fn build(self) -> Result<AccountSetAutoTopUpBillingResponse, BuildError> {
        Ok(AccountSetAutoTopUpBillingResponse {
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
