pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRefundLiabilityListResponseRowsItem {
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
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1SalesRefundLiabilityListResponseRowsItem {
    pub fn builder() -> PostV1SalesRefundLiabilityListResponseRowsItemBuilder {
        <PostV1SalesRefundLiabilityListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRefundLiabilityListResponseRowsItemBuilder {
    id: Option<String>,
    invoice_id: Option<String>,
    invoice_full_number: Option<String>,
    estimated: Option<String>,
    consumed: Option<String>,
    settlement_refunds: Option<String>,
    remaining: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1SalesRefundLiabilityListResponseRowsItemBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRefundLiabilityListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::id)
    /// - [`invoice_id`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::invoice_id)
    /// - [`estimated`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::estimated)
    /// - [`consumed`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::consumed)
    /// - [`settlement_refunds`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::settlement_refunds)
    /// - [`remaining`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::remaining)
    /// - [`created_at`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1SalesRefundLiabilityListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1SalesRefundLiabilityListResponseRowsItem, BuildError> {
        Ok(PostV1SalesRefundLiabilityListResponseRowsItem {
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
