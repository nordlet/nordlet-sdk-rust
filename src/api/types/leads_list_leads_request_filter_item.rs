pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListLeadsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListLeadsRequestFilterItemOp,
    pub value: ListLeadsRequestFilterItemValue,
}

impl ListLeadsRequestFilterItem {
    pub fn builder() -> ListLeadsRequestFilterItemBuilder {
        <ListLeadsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLeadsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListLeadsRequestFilterItemOp>,
    value: Option<ListLeadsRequestFilterItemValue>,
}

impl ListLeadsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListLeadsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListLeadsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLeadsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListLeadsRequestFilterItemBuilder::field)
    /// - [`op`](ListLeadsRequestFilterItemBuilder::op)
    /// - [`value`](ListLeadsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListLeadsRequestFilterItem, BuildError> {
        Ok(ListLeadsRequestFilterItem {
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
