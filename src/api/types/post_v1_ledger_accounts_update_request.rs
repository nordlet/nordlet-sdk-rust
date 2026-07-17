pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_postable: Option<bool>,
}

impl PostV1LedgerAccountsUpdateRequest {
    pub fn builder() -> PostV1LedgerAccountsUpdateRequestBuilder {
        <PostV1LedgerAccountsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
}

impl PostV1LedgerAccountsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerAccountsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateRequest, BuildError> {
        Ok(PostV1LedgerAccountsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            parent_id: self.parent_id,
            is_postable: self.is_postable,
        })
    }
}
