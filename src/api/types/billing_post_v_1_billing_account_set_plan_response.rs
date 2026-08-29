pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BillingAccountSetPlanResponse {
    pub plan: PostV1BillingAccountSetPlanResponsePlan,
    pub status: PostV1BillingAccountSetPlanResponseStatus,
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
    #[serde(rename = "monthToDate")]
    #[serde(default)]
    pub month_to_date: PostV1BillingAccountSetPlanResponseMonthToDate,
    #[serde(default)]
    pub plans: HashMap<String, PostV1BillingAccountSetPlanResponsePlansValue>,
    #[serde(rename = "topUp")]
    #[serde(default)]
    pub top_up: PostV1BillingAccountSetPlanResponseTopUp,
    #[serde(rename = "trialDays")]
    #[serde(default)]
    pub trial_days: i64,
}

impl PostV1BillingAccountSetPlanResponse {
    pub fn builder() -> PostV1BillingAccountSetPlanResponseBuilder {
        <PostV1BillingAccountSetPlanResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountSetPlanResponseBuilder {
    plan: Option<PostV1BillingAccountSetPlanResponsePlan>,
    status: Option<PostV1BillingAccountSetPlanResponseStatus>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<String>,
    first_top_up_at: Option<String>,
    last_charged_date: Option<String>,
    payments_configured: Option<bool>,
    month_to_date: Option<PostV1BillingAccountSetPlanResponseMonthToDate>,
    plans: Option<HashMap<String, PostV1BillingAccountSetPlanResponsePlansValue>>,
    top_up: Option<PostV1BillingAccountSetPlanResponseTopUp>,
    trial_days: Option<i64>,
}

impl PostV1BillingAccountSetPlanResponseBuilder {
    pub fn plan(mut self, value: PostV1BillingAccountSetPlanResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1BillingAccountSetPlanResponseStatus) -> Self {
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

    pub fn month_to_date(mut self, value: PostV1BillingAccountSetPlanResponseMonthToDate) -> Self {
        self.month_to_date = Some(value);
        self
    }

    pub fn plans(
        mut self,
        value: HashMap<String, PostV1BillingAccountSetPlanResponsePlansValue>,
    ) -> Self {
        self.plans = Some(value);
        self
    }

    pub fn top_up(mut self, value: PostV1BillingAccountSetPlanResponseTopUp) -> Self {
        self.top_up = Some(value);
        self
    }

    pub fn trial_days(mut self, value: i64) -> Self {
        self.trial_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingAccountSetPlanResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](PostV1BillingAccountSetPlanResponseBuilder::plan)
    /// - [`status`](PostV1BillingAccountSetPlanResponseBuilder::status)
    /// - [`balance_cents`](PostV1BillingAccountSetPlanResponseBuilder::balance_cents)
    /// - [`payments_configured`](PostV1BillingAccountSetPlanResponseBuilder::payments_configured)
    /// - [`month_to_date`](PostV1BillingAccountSetPlanResponseBuilder::month_to_date)
    /// - [`plans`](PostV1BillingAccountSetPlanResponseBuilder::plans)
    /// - [`top_up`](PostV1BillingAccountSetPlanResponseBuilder::top_up)
    /// - [`trial_days`](PostV1BillingAccountSetPlanResponseBuilder::trial_days)
    pub fn build(self) -> Result<PostV1BillingAccountSetPlanResponse, BuildError> {
        Ok(PostV1BillingAccountSetPlanResponse {
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
