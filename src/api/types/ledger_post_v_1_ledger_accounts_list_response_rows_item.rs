pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1LedgerAccountsListResponseRowsItemType,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(default)]
    pub is_postable: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1LedgerAccountsListResponseRowsItem {
    pub fn builder() -> PostV1LedgerAccountsListResponseRowsItemBuilder {
        <PostV1LedgerAccountsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1LedgerAccountsListResponseRowsItemType>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
    created_at: Option<String>,
}

impl PostV1LedgerAccountsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1LedgerAccountsListResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn is_postable(mut self, value: bool) -> Self {
        self.is_postable = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerAccountsListResponseRowsItemBuilder::id)
    /// - [`code`](PostV1LedgerAccountsListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1LedgerAccountsListResponseRowsItemBuilder::name)
    /// - [`r#type`](PostV1LedgerAccountsListResponseRowsItemBuilder::r#type)
    /// - [`is_postable`](PostV1LedgerAccountsListResponseRowsItemBuilder::is_postable)
    /// - [`created_at`](PostV1LedgerAccountsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1LedgerAccountsListResponseRowsItem, BuildError> {
        Ok(PostV1LedgerAccountsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            parent_id: self.parent_id,
            is_postable: self
                .is_postable
                .ok_or_else(|| BuildError::missing_field("is_postable"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
