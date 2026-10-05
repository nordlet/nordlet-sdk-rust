pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundLiabilityListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RefundLiabilityListSalesRequestFilterItemOp,
    pub value: RefundLiabilityListSalesRequestFilterItemValue,
}

impl RefundLiabilityListSalesRequestFilterItem {
    pub fn builder() -> RefundLiabilityListSalesRequestFilterItemBuilder {
        <RefundLiabilityListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLiabilityListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RefundLiabilityListSalesRequestFilterItemOp>,
    value: Option<RefundLiabilityListSalesRequestFilterItemValue>,
}

impl RefundLiabilityListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RefundLiabilityListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RefundLiabilityListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLiabilityListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RefundLiabilityListSalesRequestFilterItemBuilder::field)
    /// - [`op`](RefundLiabilityListSalesRequestFilterItemBuilder::op)
    /// - [`value`](RefundLiabilityListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RefundLiabilityListSalesRequestFilterItem, BuildError> {
        Ok(RefundLiabilityListSalesRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
