pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseBilling {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub plan: String,
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
}

impl ExportAccountResponseBilling {
    pub fn builder() -> ExportAccountResponseBillingBuilder {
        <ExportAccountResponseBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseBillingBuilder {
    status: Option<String>,
    plan: Option<String>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<DateTime<FixedOffset>>,
    first_top_up_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseBillingBuilder {
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

    pub fn trial_ends_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.trial_ends_at = Some(value);
        self
    }

    pub fn first_top_up_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_top_up_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseBilling`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ExportAccountResponseBillingBuilder::status)
    /// - [`plan`](ExportAccountResponseBillingBuilder::plan)
    /// - [`balance_cents`](ExportAccountResponseBillingBuilder::balance_cents)
    pub fn build(self) -> Result<ExportAccountResponseBilling, BuildError> {
        Ok(ExportAccountResponseBilling {
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
