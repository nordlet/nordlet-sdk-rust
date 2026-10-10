pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsPostBankResponseSummary {
    #[serde(rename = "receivableApplied")]
    #[serde(default)]
    pub receivable_applied: String,
    #[serde(rename = "commissionAmount")]
    #[serde(default)]
    pub commission_amount: String,
    #[serde(rename = "sellerAmount")]
    #[serde(default)]
    pub seller_amount: String,
    #[serde(rename = "feeAmount")]
    #[serde(default)]
    pub fee_amount: String,
    #[serde(rename = "suspenseAmount")]
    #[serde(default)]
    pub suspense_amount: String,
    #[serde(rename = "clearedAmount")]
    #[serde(default)]
    pub cleared_amount: String,
    #[serde(rename = "fxRate")]
    #[serde(default)]
    pub fx_rate: String,
    #[serde(rename = "exchangeDifference")]
    #[serde(default)]
    pub exchange_difference: String,
}

impl SettlementsPostBankResponseSummary {
    pub fn builder() -> SettlementsPostBankResponseSummaryBuilder {
        <SettlementsPostBankResponseSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsPostBankResponseSummaryBuilder {
    receivable_applied: Option<String>,
    commission_amount: Option<String>,
    seller_amount: Option<String>,
    fee_amount: Option<String>,
    suspense_amount: Option<String>,
    cleared_amount: Option<String>,
    fx_rate: Option<String>,
    exchange_difference: Option<String>,
}

impl SettlementsPostBankResponseSummaryBuilder {
    pub fn receivable_applied(mut self, value: impl Into<String>) -> Self {
        self.receivable_applied = Some(value.into());
        self
    }

    pub fn commission_amount(mut self, value: impl Into<String>) -> Self {
        self.commission_amount = Some(value.into());
        self
    }

    pub fn seller_amount(mut self, value: impl Into<String>) -> Self {
        self.seller_amount = Some(value.into());
        self
    }

    pub fn fee_amount(mut self, value: impl Into<String>) -> Self {
        self.fee_amount = Some(value.into());
        self
    }

    pub fn suspense_amount(mut self, value: impl Into<String>) -> Self {
        self.suspense_amount = Some(value.into());
        self
    }

    pub fn cleared_amount(mut self, value: impl Into<String>) -> Self {
        self.cleared_amount = Some(value.into());
        self
    }

    pub fn fx_rate(mut self, value: impl Into<String>) -> Self {
        self.fx_rate = Some(value.into());
        self
    }

    pub fn exchange_difference(mut self, value: impl Into<String>) -> Self {
        self.exchange_difference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SettlementsPostBankResponseSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`receivable_applied`](SettlementsPostBankResponseSummaryBuilder::receivable_applied)
    /// - [`commission_amount`](SettlementsPostBankResponseSummaryBuilder::commission_amount)
    /// - [`seller_amount`](SettlementsPostBankResponseSummaryBuilder::seller_amount)
    /// - [`fee_amount`](SettlementsPostBankResponseSummaryBuilder::fee_amount)
    /// - [`suspense_amount`](SettlementsPostBankResponseSummaryBuilder::suspense_amount)
    /// - [`cleared_amount`](SettlementsPostBankResponseSummaryBuilder::cleared_amount)
    /// - [`fx_rate`](SettlementsPostBankResponseSummaryBuilder::fx_rate)
    /// - [`exchange_difference`](SettlementsPostBankResponseSummaryBuilder::exchange_difference)
    pub fn build(self) -> Result<SettlementsPostBankResponseSummary, BuildError> {
        Ok(SettlementsPostBankResponseSummary {
            receivable_applied: self
                .receivable_applied
                .ok_or_else(|| BuildError::missing_field("receivable_applied"))?,
            commission_amount: self
                .commission_amount
                .ok_or_else(|| BuildError::missing_field("commission_amount"))?,
            seller_amount: self
                .seller_amount
                .ok_or_else(|| BuildError::missing_field("seller_amount"))?,
            fee_amount: self
                .fee_amount
                .ok_or_else(|| BuildError::missing_field("fee_amount"))?,
            suspense_amount: self
                .suspense_amount
                .ok_or_else(|| BuildError::missing_field("suspense_amount"))?,
            cleared_amount: self
                .cleared_amount
                .ok_or_else(|| BuildError::missing_field("cleared_amount"))?,
            fx_rate: self
                .fx_rate
                .ok_or_else(|| BuildError::missing_field("fx_rate"))?,
            exchange_difference: self
                .exchange_difference
                .ok_or_else(|| BuildError::missing_field("exchange_difference"))?,
        })
    }
}
