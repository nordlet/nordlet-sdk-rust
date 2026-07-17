pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1LedgerAccountsUpdateResponseType,
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

impl PostV1LedgerAccountsUpdateResponse {
    pub fn builder() -> PostV1LedgerAccountsUpdateResponseBuilder {
        <PostV1LedgerAccountsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1LedgerAccountsUpdateResponseType>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
    created_at: Option<String>,
}

impl PostV1LedgerAccountsUpdateResponseBuilder {
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

    pub fn r#type(mut self, value: PostV1LedgerAccountsUpdateResponseType) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerAccountsUpdateResponseBuilder::id)
    /// - [`code`](PostV1LedgerAccountsUpdateResponseBuilder::code)
    /// - [`name`](PostV1LedgerAccountsUpdateResponseBuilder::name)
    /// - [`r#type`](PostV1LedgerAccountsUpdateResponseBuilder::r#type)
    /// - [`is_postable`](PostV1LedgerAccountsUpdateResponseBuilder::is_postable)
    /// - [`created_at`](PostV1LedgerAccountsUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateResponse, BuildError> {
        Ok(PostV1LedgerAccountsUpdateResponse {
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
