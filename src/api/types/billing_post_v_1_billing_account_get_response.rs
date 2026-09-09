pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BillingAccountGetResponse {
    pub plan: PostV1BillingAccountGetResponsePlan,
    pub status: PostV1BillingAccountGetResponseStatus,
    #[serde(rename = "balanceCents")]
    #[serde(default)]
    pub balance_cents: i64,
    #[serde(rename = "trialEndsAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_ends_at: Option<String>,
    #[serde(rename = "firstTopUpAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_top_up_at: Option<String>,
    #[serde(rename = "lastChargedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_charged_date: Option<String>,
    #[serde(rename = "paymentsConfigured")]
    #[serde(default)]
    pub payments_configured: bool,
    #[serde(rename = "hasPaymentAccount")]
    #[serde(default)]
    pub has_payment_account: bool,
    #[serde(rename = "hasSubscription")]
    #[serde(default)]
    pub has_subscription: bool,
    #[serde(rename = "monthToDate")]
    #[serde(default)]
    pub month_to_date: PostV1BillingAccountGetResponseMonthToDate,
    #[serde(default)]
    pub plans: HashMap<String, PostV1BillingAccountGetResponsePlansValue>,
    #[serde(rename = "topUp")]
    #[serde(default)]
    pub top_up: PostV1BillingAccountGetResponseTopUp,
    #[serde(rename = "trialDays")]
    #[serde(default)]
    pub trial_days: i64,
}

impl PostV1BillingAccountGetResponse {
    pub fn builder() -> PostV1BillingAccountGetResponseBuilder {
        <PostV1BillingAccountGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountGetResponseBuilder {
    plan: Option<PostV1BillingAccountGetResponsePlan>,
    status: Option<PostV1BillingAccountGetResponseStatus>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<String>,
    first_top_up_at: Option<String>,
    last_charged_date: Option<String>,
    payments_configured: Option<bool>,
    has_payment_account: Option<bool>,
    has_subscription: Option<bool>,
    month_to_date: Option<PostV1BillingAccountGetResponseMonthToDate>,
    plans: Option<HashMap<String, PostV1BillingAccountGetResponsePlansValue>>,
    top_up: Option<PostV1BillingAccountGetResponseTopUp>,
    trial_days: Option<i64>,
}

impl PostV1BillingAccountGetResponseBuilder {
    pub fn plan(mut self, value: PostV1BillingAccountGetResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1BillingAccountGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn balance_cents(mut self, value: i64) -> Self {
        self.balance_cents = Some(value);
        self
    }

    pub fn trial_ends_at(mut self, value: impl Into<String>) -> Self {
        self.trial_ends_at = Some(value.into());
        self
    }

    pub fn first_top_up_at(mut self, value: impl Into<String>) -> Self {
        self.first_top_up_at = Some(value.into());
        self
    }

    pub fn last_charged_date(mut self, value: impl Into<String>) -> Self {
        self.last_charged_date = Some(value.into());
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

    pub fn month_to_date(mut self, value: PostV1BillingAccountGetResponseMonthToDate) -> Self {
        self.month_to_date = Some(value);
        self
    }

    pub fn plans(
        mut self,
        value: HashMap<String, PostV1BillingAccountGetResponsePlansValue>,
    ) -> Self {
        self.plans = Some(value);
        self
    }

    pub fn top_up(mut self, value: PostV1BillingAccountGetResponseTopUp) -> Self {
        self.top_up = Some(value);
        self
    }

    pub fn trial_days(mut self, value: i64) -> Self {
        self.trial_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingAccountGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](PostV1BillingAccountGetResponseBuilder::plan)
    /// - [`status`](PostV1BillingAccountGetResponseBuilder::status)
    /// - [`balance_cents`](PostV1BillingAccountGetResponseBuilder::balance_cents)
    /// - [`payments_configured`](PostV1BillingAccountGetResponseBuilder::payments_configured)
    /// - [`has_payment_account`](PostV1BillingAccountGetResponseBuilder::has_payment_account)
    /// - [`has_subscription`](PostV1BillingAccountGetResponseBuilder::has_subscription)
    /// - [`month_to_date`](PostV1BillingAccountGetResponseBuilder::month_to_date)
    /// - [`plans`](PostV1BillingAccountGetResponseBuilder::plans)
    /// - [`top_up`](PostV1BillingAccountGetResponseBuilder::top_up)
    /// - [`trial_days`](PostV1BillingAccountGetResponseBuilder::trial_days)
    pub fn build(self) -> Result<PostV1BillingAccountGetResponse, BuildError> {
        Ok(PostV1BillingAccountGetResponse {
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
