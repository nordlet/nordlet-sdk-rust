pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundLiabilityListSalesResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "invoiceFullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_full_number: Option<String>,
    #[serde(default)]
    pub estimated: String,
    #[serde(default)]
    pub consumed: String,
    #[serde(rename = "settlementRefunds")]
    #[serde(default)]
    pub settlement_refunds: String,
    #[serde(default)]
    pub remaining: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl RefundLiabilityListSalesResponseRowsItem {
    pub fn builder() -> RefundLiabilityListSalesResponseRowsItemBuilder {
        <RefundLiabilityListSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLiabilityListSalesResponseRowsItemBuilder {
    id: Option<String>,
    invoice_id: Option<String>,
    invoice_full_number: Option<String>,
    estimated: Option<String>,
    consumed: Option<String>,
    settlement_refunds: Option<String>,
    remaining: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RefundLiabilityListSalesResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn invoice_full_number(mut self, value: impl Into<String>) -> Self {
        self.invoice_full_number = Some(value.into());
        self
    }

    pub fn estimated(mut self, value: impl Into<String>) -> Self {
        self.estimated = Some(value.into());
        self
    }

    pub fn consumed(mut self, value: impl Into<String>) -> Self {
        self.consumed = Some(value.into());
        self
    }

    pub fn settlement_refunds(mut self, value: impl Into<String>) -> Self {
        self.settlement_refunds = Some(value.into());
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLiabilityListSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundLiabilityListSalesResponseRowsItemBuilder::id)
    /// - [`invoice_id`](RefundLiabilityListSalesResponseRowsItemBuilder::invoice_id)
    /// - [`estimated`](RefundLiabilityListSalesResponseRowsItemBuilder::estimated)
    /// - [`consumed`](RefundLiabilityListSalesResponseRowsItemBuilder::consumed)
    /// - [`settlement_refunds`](RefundLiabilityListSalesResponseRowsItemBuilder::settlement_refunds)
    /// - [`remaining`](RefundLiabilityListSalesResponseRowsItemBuilder::remaining)
    /// - [`created_at`](RefundLiabilityListSalesResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](RefundLiabilityListSalesResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<RefundLiabilityListSalesResponseRowsItem, BuildError> {
        Ok(RefundLiabilityListSalesResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            invoice_full_number: self.invoice_full_number,
            estimated: self
                .estimated
                .ok_or_else(|| BuildError::missing_field("estimated"))?,
            consumed: self
                .consumed
                .ok_or_else(|| BuildError::missing_field("consumed"))?,
            settlement_refunds: self
                .settlement_refunds
                .ok_or_else(|| BuildError::missing_field("settlement_refunds"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
