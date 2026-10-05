pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListPartnersRequestFilterItemOp,
    pub value: ListPartnersRequestFilterItemValue,
}

impl ListPartnersRequestFilterItem {
    pub fn builder() -> ListPartnersRequestFilterItemBuilder {
        <ListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListPartnersRequestFilterItemOp>,
    value: Option<ListPartnersRequestFilterItemValue>,
}

impl ListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](ListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](ListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListPartnersRequestFilterItem, BuildError> {
        Ok(ListPartnersRequestFilterItem {
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
