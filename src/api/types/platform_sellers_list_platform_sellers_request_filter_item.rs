pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPlatformSellersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListPlatformSellersRequestFilterItemOp,
    pub value: ListPlatformSellersRequestFilterItemValue,
}

impl ListPlatformSellersRequestFilterItem {
    pub fn builder() -> ListPlatformSellersRequestFilterItemBuilder {
        <ListPlatformSellersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPlatformSellersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListPlatformSellersRequestFilterItemOp>,
    value: Option<ListPlatformSellersRequestFilterItemValue>,
}

impl ListPlatformSellersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListPlatformSellersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListPlatformSellersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPlatformSellersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListPlatformSellersRequestFilterItemBuilder::field)
    /// - [`op`](ListPlatformSellersRequestFilterItemBuilder::op)
    /// - [`value`](ListPlatformSellersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListPlatformSellersRequestFilterItem, BuildError> {
        Ok(ListPlatformSellersRequestFilterItem {
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
