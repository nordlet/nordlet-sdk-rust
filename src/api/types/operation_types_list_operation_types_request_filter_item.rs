pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOperationTypesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListOperationTypesRequestFilterItemOp,
    pub value: ListOperationTypesRequestFilterItemValue,
}

impl ListOperationTypesRequestFilterItem {
    pub fn builder() -> ListOperationTypesRequestFilterItemBuilder {
        <ListOperationTypesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOperationTypesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListOperationTypesRequestFilterItemOp>,
    value: Option<ListOperationTypesRequestFilterItemValue>,
}

impl ListOperationTypesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListOperationTypesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListOperationTypesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOperationTypesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListOperationTypesRequestFilterItemBuilder::field)
    /// - [`op`](ListOperationTypesRequestFilterItemBuilder::op)
    /// - [`value`](ListOperationTypesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListOperationTypesRequestFilterItem, BuildError> {
        Ok(ListOperationTypesRequestFilterItem {
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
