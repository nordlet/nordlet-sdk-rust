pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CashOrdersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1CashOrdersListRequestSortItemDir>,
}

impl PostV1CashOrdersListRequestSortItem {
    pub fn builder() -> PostV1CashOrdersListRequestSortItemBuilder {
        <PostV1CashOrdersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashOrdersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1CashOrdersListRequestSortItemDir>,
}

impl PostV1CashOrdersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1CashOrdersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashOrdersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1CashOrdersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1CashOrdersListRequestSortItem, BuildError> {
        Ok(PostV1CashOrdersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
