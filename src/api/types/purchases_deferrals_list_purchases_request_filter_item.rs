pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeferralsListPurchasesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: DeferralsListPurchasesRequestFilterItemOp,
    pub value: DeferralsListPurchasesRequestFilterItemValue,
}

impl DeferralsListPurchasesRequestFilterItem {
    pub fn builder() -> DeferralsListPurchasesRequestFilterItemBuilder {
        <DeferralsListPurchasesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeferralsListPurchasesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<DeferralsListPurchasesRequestFilterItemOp>,
    value: Option<DeferralsListPurchasesRequestFilterItemValue>,
}

impl DeferralsListPurchasesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: DeferralsListPurchasesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: DeferralsListPurchasesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeferralsListPurchasesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DeferralsListPurchasesRequestFilterItemBuilder::field)
    /// - [`op`](DeferralsListPurchasesRequestFilterItemBuilder::op)
    /// - [`value`](DeferralsListPurchasesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<DeferralsListPurchasesRequestFilterItem, BuildError> {
        Ok(DeferralsListPurchasesRequestFilterItem {
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
