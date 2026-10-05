pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAuditRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListAuditRequestFilterItemOp,
    pub value: ListAuditRequestFilterItemValue,
}

impl ListAuditRequestFilterItem {
    pub fn builder() -> ListAuditRequestFilterItemBuilder {
        <ListAuditRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuditRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListAuditRequestFilterItemOp>,
    value: Option<ListAuditRequestFilterItemValue>,
}

impl ListAuditRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListAuditRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListAuditRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuditRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListAuditRequestFilterItemBuilder::field)
    /// - [`op`](ListAuditRequestFilterItemBuilder::op)
    /// - [`value`](ListAuditRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListAuditRequestFilterItem, BuildError> {
        Ok(ListAuditRequestFilterItem {
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
