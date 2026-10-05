pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFilesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListFilesRequestFilterItemOp,
    pub value: ListFilesRequestFilterItemValue,
}

impl ListFilesRequestFilterItem {
    pub fn builder() -> ListFilesRequestFilterItemBuilder {
        <ListFilesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFilesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListFilesRequestFilterItemOp>,
    value: Option<ListFilesRequestFilterItemValue>,
}

impl ListFilesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListFilesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListFilesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFilesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListFilesRequestFilterItemBuilder::field)
    /// - [`op`](ListFilesRequestFilterItemBuilder::op)
    /// - [`value`](ListFilesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListFilesRequestFilterItem, BuildError> {
        Ok(ListFilesRequestFilterItem {
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
