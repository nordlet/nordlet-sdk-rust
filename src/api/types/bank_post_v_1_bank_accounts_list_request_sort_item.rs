pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankAccountsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1BankAccountsListRequestSortItemDir>,
}

impl PostV1BankAccountsListRequestSortItem {
    pub fn builder() -> PostV1BankAccountsListRequestSortItemBuilder {
        <PostV1BankAccountsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankAccountsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1BankAccountsListRequestSortItemDir>,
}

impl PostV1BankAccountsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1BankAccountsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankAccountsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankAccountsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1BankAccountsListRequestSortItem, BuildError> {
        Ok(PostV1BankAccountsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
