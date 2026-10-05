pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedsConnectionsListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: FeedsConnectionsListBankRequestFilterItemOp,
    pub value: FeedsConnectionsListBankRequestFilterItemValue,
}

impl FeedsConnectionsListBankRequestFilterItem {
    pub fn builder() -> FeedsConnectionsListBankRequestFilterItemBuilder {
        <FeedsConnectionsListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<FeedsConnectionsListBankRequestFilterItemOp>,
    value: Option<FeedsConnectionsListBankRequestFilterItemValue>,
}

impl FeedsConnectionsListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: FeedsConnectionsListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: FeedsConnectionsListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](FeedsConnectionsListBankRequestFilterItemBuilder::field)
    /// - [`op`](FeedsConnectionsListBankRequestFilterItemBuilder::op)
    /// - [`value`](FeedsConnectionsListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<FeedsConnectionsListBankRequestFilterItem, BuildError> {
        Ok(FeedsConnectionsListBankRequestFilterItem {
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
