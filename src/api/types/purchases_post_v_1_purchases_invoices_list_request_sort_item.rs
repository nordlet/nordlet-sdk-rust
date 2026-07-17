pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PurchasesInvoicesListRequestSortItemDir>,
}

impl PostV1PurchasesInvoicesListRequestSortItem {
    pub fn builder() -> PostV1PurchasesInvoicesListRequestSortItemBuilder {
        <PostV1PurchasesInvoicesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PurchasesInvoicesListRequestSortItemDir>,
}

impl PostV1PurchasesInvoicesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PurchasesInvoicesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PurchasesInvoicesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesListRequestSortItem, BuildError> {
        Ok(PostV1PurchasesInvoicesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
