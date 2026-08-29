pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountMeResponseBilling {
    pub status: PostV1AccountMeResponseBillingStatus,
    #[serde(default)]
    pub plan: String,
    #[serde(rename = "balanceCents")]
    #[serde(default)]
    pub balance_cents: i64,
    #[serde(rename = "trialEndsAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_ends_at: Option<String>,
}

impl PostV1AccountMeResponseBilling {
    pub fn builder() -> PostV1AccountMeResponseBillingBuilder {
        <PostV1AccountMeResponseBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMeResponseBillingBuilder {
    status: Option<PostV1AccountMeResponseBillingStatus>,
    plan: Option<String>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<String>,
}

impl PostV1AccountMeResponseBillingBuilder {
    pub fn status(mut self, value: PostV1AccountMeResponseBillingStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn plan(mut self, value: impl Into<String>) -> Self {
        self.plan = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1AccountMeResponseBilling`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PostV1AccountMeResponseBillingBuilder::status)
    /// - [`plan`](PostV1AccountMeResponseBillingBuilder::plan)
    /// - [`balance_cents`](PostV1AccountMeResponseBillingBuilder::balance_cents)
    pub fn build(self) -> Result<PostV1AccountMeResponseBilling, BuildError> {
        Ok(PostV1AccountMeResponseBilling {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            balance_cents: self
                .balance_cents
                .ok_or_else(|| BuildError::missing_field("balance_cents"))?,
            trial_ends_at: self.trial_ends_at,
        })
    }
}
