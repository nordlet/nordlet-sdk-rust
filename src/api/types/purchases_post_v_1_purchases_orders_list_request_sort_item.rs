pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PurchasesOrdersListRequestSortItemDir>,
}

impl PostV1PurchasesOrdersListRequestSortItem {
    pub fn builder() -> PostV1PurchasesOrdersListRequestSortItemBuilder {
        <PostV1PurchasesOrdersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PurchasesOrdersListRequestSortItemDir>,
}

impl PostV1PurchasesOrdersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PurchasesOrdersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PurchasesOrdersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PurchasesOrdersListRequestSortItem, BuildError> {
        Ok(PostV1PurchasesOrdersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
