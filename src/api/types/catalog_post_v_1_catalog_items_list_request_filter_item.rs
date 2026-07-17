pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1CatalogItemsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1CatalogItemsListRequestFilterItemOp,
    pub value: PostV1CatalogItemsListRequestFilterItemValue,
}

impl PostV1CatalogItemsListRequestFilterItem {
    pub fn builder() -> PostV1CatalogItemsListRequestFilterItemBuilder {
        <PostV1CatalogItemsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1CatalogItemsListRequestFilterItemOp>,
    value: Option<PostV1CatalogItemsListRequestFilterItemValue>,
}

impl PostV1CatalogItemsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1CatalogItemsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1CatalogItemsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1CatalogItemsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1CatalogItemsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1CatalogItemsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1CatalogItemsListRequestFilterItem, BuildError> {
        Ok(PostV1CatalogItemsListRequestFilterItem {
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
