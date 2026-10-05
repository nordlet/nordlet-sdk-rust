pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActsListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ActsListSalesRequestFilterItemOp,
    pub value: ActsListSalesRequestFilterItemValue,
}

impl ActsListSalesRequestFilterItem {
    pub fn builder() -> ActsListSalesRequestFilterItemBuilder {
        <ActsListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ActsListSalesRequestFilterItemOp>,
    value: Option<ActsListSalesRequestFilterItemValue>,
}

impl ActsListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ActsListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ActsListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActsListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ActsListSalesRequestFilterItemBuilder::field)
    /// - [`op`](ActsListSalesRequestFilterItemBuilder::op)
    /// - [`value`](ActsListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ActsListSalesRequestFilterItem, BuildError> {
        Ok(ActsListSalesRequestFilterItem {
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
