pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesReceiptsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PurchasesReceiptsListRequestSortItemDir>,
}

impl PostV1PurchasesReceiptsListRequestSortItem {
    pub fn builder() -> PostV1PurchasesReceiptsListRequestSortItemBuilder {
        <PostV1PurchasesReceiptsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PurchasesReceiptsListRequestSortItemDir>,
}

impl PostV1PurchasesReceiptsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PurchasesReceiptsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PurchasesReceiptsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsListRequestSortItem, BuildError> {
        Ok(PostV1PurchasesReceiptsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
