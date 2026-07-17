pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1BankTransactionsListRequestSortItemDir>,
}

impl PostV1BankTransactionsListRequestSortItem {
    pub fn builder() -> PostV1BankTransactionsListRequestSortItemBuilder {
        <PostV1BankTransactionsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1BankTransactionsListRequestSortItemDir>,
}

impl PostV1BankTransactionsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1BankTransactionsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankTransactionsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1BankTransactionsListRequestSortItem, BuildError> {
        Ok(PostV1BankTransactionsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
