pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseBillingAccountsItem {
    pub site: ExportAccountResponseBillingAccountsItemSite,
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

impl ExportAccountResponseBillingAccountsItem {
    pub fn builder() -> ExportAccountResponseBillingAccountsItemBuilder {
        <ExportAccountResponseBillingAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseBillingAccountsItemBuilder {
    site: Option<ExportAccountResponseBillingAccountsItemSite>,
    status: Option<String>,
    plan: Option<String>,
    balance_cents: Option<i64>,
    trial_ends_at: Option<DateTime<FixedOffset>>,
    first_top_up_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseBillingAccountsItemBuilder {
    pub fn site(mut self, value: ExportAccountResponseBillingAccountsItemSite) -> Self {
        self.site = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`ExportAccountResponseBillingAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`site`](ExportAccountResponseBillingAccountsItemBuilder::site)
    /// - [`status`](ExportAccountResponseBillingAccountsItemBuilder::status)
    /// - [`plan`](ExportAccountResponseBillingAccountsItemBuilder::plan)
    /// - [`balance_cents`](ExportAccountResponseBillingAccountsItemBuilder::balance_cents)
    pub fn build(self) -> Result<ExportAccountResponseBillingAccountsItem, BuildError> {
        Ok(ExportAccountResponseBillingAccountsItem {
            site: self.site.ok_or_else(|| BuildError::missing_field("site"))?,
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
