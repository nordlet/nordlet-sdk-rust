pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponseBilling {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub plan: String,
    #[serde(rename = "balanceCents")]
    #[serde(default)]
    pub balance_cents: i64,
    #[serde(rename = "trialEndsAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_ends_at: Option<String>,
    #[serde(rename = "firstTopUpAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_top_up_at: Option<String>,
}

impl PostV1AccountExportResponseBilling {
    pub fn builder() -> PostV1AccountExportResponseBillingBuilder {
        <PostV1AccountExportResponseBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseBillingBuilder {
    status: Option<String>,
    plan: Option<String>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<String>,
    first_top_up_at: Option<String>,
}

impl PostV1AccountExportResponseBillingBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    pub fn first_top_up_at(mut self, value: impl Into<String>) -> Self {
        self.first_top_up_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountExportResponseBilling`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PostV1AccountExportResponseBillingBuilder::status)
    /// - [`plan`](PostV1AccountExportResponseBillingBuilder::plan)
    /// - [`balance_cents`](PostV1AccountExportResponseBillingBuilder::balance_cents)
    pub fn build(self) -> Result<PostV1AccountExportResponseBilling, BuildError> {
        Ok(PostV1AccountExportResponseBilling {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            balance_cents: self
                .balance_cents
                .ok_or_else(|| BuildError::missing_field("balance_cents"))?,
            trial_ends_at: self.trial_ends_at,
            first_top_up_at: self.first_top_up_at,
        })
    }
}
