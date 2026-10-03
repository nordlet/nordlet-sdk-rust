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
    #[serde(rename = "payerUserId")]
    #[serde(default)]
    pub payer_user_id: String,
    #[serde(rename = "payerEmail")]
    #[serde(default)]
    pub payer_email: String,
    #[serde(rename = "isPayer")]
    #[serde(default)]
    pub is_payer: bool,
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
    payer_user_id: Option<String>,
    payer_email: Option<String>,
    is_payer: Option<bool>,
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

    pub fn payer_user_id(mut self, value: impl Into<String>) -> Self {
        self.payer_user_id = Some(value.into());
        self
    }

    pub fn payer_email(mut self, value: impl Into<String>) -> Self {
        self.payer_email = Some(value.into());
        self
    }

    pub fn is_payer(mut self, value: bool) -> Self {
        self.is_payer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMeResponseBilling`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PostV1AccountMeResponseBillingBuilder::status)
    /// - [`plan`](PostV1AccountMeResponseBillingBuilder::plan)
    /// - [`balance_cents`](PostV1AccountMeResponseBillingBuilder::balance_cents)
    /// - [`payer_user_id`](PostV1AccountMeResponseBillingBuilder::payer_user_id)
    /// - [`payer_email`](PostV1AccountMeResponseBillingBuilder::payer_email)
    /// - [`is_payer`](PostV1AccountMeResponseBillingBuilder::is_payer)
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
            payer_user_id: self
                .payer_user_id
                .ok_or_else(|| BuildError::missing_field("payer_user_id"))?,
            payer_email: self
                .payer_email
                .ok_or_else(|| BuildError::missing_field("payer_email"))?,
            is_payer: self
                .is_payer
                .ok_or_else(|| BuildError::missing_field("is_payer"))?,
        })
    }
}
