pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestAccountsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1MigrationBooksImportRequestAccountsItemType,
    #[serde(rename = "parentCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_code: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_postable: Option<bool>,
}

impl PostV1MigrationBooksImportRequestAccountsItem {
    pub fn builder() -> PostV1MigrationBooksImportRequestAccountsItemBuilder {
        <PostV1MigrationBooksImportRequestAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestAccountsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1MigrationBooksImportRequestAccountsItemType>,
    parent_code: Option<String>,
    is_postable: Option<bool>,
}

impl PostV1MigrationBooksImportRequestAccountsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1MigrationBooksImportRequestAccountsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn parent_code(mut self, value: impl Into<String>) -> Self {
        self.parent_code = Some(value.into());
        self
    }

    pub fn is_postable(mut self, value: bool) -> Self {
        self.is_postable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1MigrationBooksImportRequestAccountsItemBuilder::code)
    /// - [`name`](PostV1MigrationBooksImportRequestAccountsItemBuilder::name)
    /// - [`r#type`](PostV1MigrationBooksImportRequestAccountsItemBuilder::r#type)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestAccountsItem, BuildError> {
        Ok(PostV1MigrationBooksImportRequestAccountsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            parent_code: self.parent_code,
            is_postable: self.is_postable,
        })
    }
}
