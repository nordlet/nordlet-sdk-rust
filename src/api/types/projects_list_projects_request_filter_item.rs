pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListProjectsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListProjectsRequestFilterItemOp,
    pub value: ListProjectsRequestFilterItemValue,
}

impl ListProjectsRequestFilterItem {
    pub fn builder() -> ListProjectsRequestFilterItemBuilder {
        <ListProjectsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProjectsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListProjectsRequestFilterItemOp>,
    value: Option<ListProjectsRequestFilterItemValue>,
}

impl ListProjectsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListProjectsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListProjectsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProjectsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListProjectsRequestFilterItemBuilder::field)
    /// - [`op`](ListProjectsRequestFilterItemBuilder::op)
    /// - [`value`](ListProjectsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListProjectsRequestFilterItem, BuildError> {
        Ok(ListProjectsRequestFilterItem {
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
