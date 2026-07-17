pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1SalesInvoicesListRequestSortItemDir>,
}

impl PostV1SalesInvoicesListRequestSortItem {
    pub fn builder() -> PostV1SalesInvoicesListRequestSortItemBuilder {
        <PostV1SalesInvoicesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1SalesInvoicesListRequestSortItemDir>,
}

impl PostV1SalesInvoicesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1SalesInvoicesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesInvoicesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1SalesInvoicesListRequestSortItem, BuildError> {
        Ok(PostV1SalesInvoicesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
