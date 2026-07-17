pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersBankAccountsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersBankAccountsListRequestSortItemDir>,
}

impl PostV1PartnersBankAccountsListRequestSortItem {
    pub fn builder() -> PostV1PartnersBankAccountsListRequestSortItemBuilder {
        <PostV1PartnersBankAccountsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersBankAccountsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersBankAccountsListRequestSortItemDir>,
}

impl PostV1PartnersBankAccountsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersBankAccountsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersBankAccountsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersBankAccountsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersBankAccountsListRequestSortItem, BuildError> {
        Ok(PostV1PartnersBankAccountsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
