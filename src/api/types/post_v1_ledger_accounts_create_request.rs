pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1LedgerAccountsCreateRequestType,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_postable: Option<bool>,
}

impl PostV1LedgerAccountsCreateRequest {
    pub fn builder() -> PostV1LedgerAccountsCreateRequestBuilder {
        <PostV1LedgerAccountsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1LedgerAccountsCreateRequestType>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
}

impl PostV1LedgerAccountsCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1LedgerAccountsCreateRequestType) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerAccountsCreateRequestBuilder::code)
    /// - [`name`](PostV1LedgerAccountsCreateRequestBuilder::name)
    /// - [`r#type`](PostV1LedgerAccountsCreateRequestBuilder::r#type)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateRequest, BuildError> {
        Ok(PostV1LedgerAccountsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            parent_id: self.parent_id,
            is_postable: self.is_postable,
        })
    }
}
