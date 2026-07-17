pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountMeResponseCompaniesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "isSandbox")]
    #[serde(default)]
    pub is_sandbox: bool,
    pub status: PostV1AccountMeResponseCompaniesItemStatus,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl PostV1AccountMeResponseCompaniesItem {
    pub fn builder() -> PostV1AccountMeResponseCompaniesItemBuilder {
        <PostV1AccountMeResponseCompaniesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMeResponseCompaniesItemBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    role: Option<String>,
    is_sandbox: Option<bool>,
    status: Option<PostV1AccountMeResponseCompaniesItemStatus>,
    deleted_at: Option<String>,
}

impl PostV1AccountMeResponseCompaniesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1AccountMeResponseCompaniesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: impl Into<String>) -> Self {
        self.deleted_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMeResponseCompaniesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountMeResponseCompaniesItemBuilder::id)
    /// - [`name`](PostV1AccountMeResponseCompaniesItemBuilder::name)
    /// - [`role`](PostV1AccountMeResponseCompaniesItemBuilder::role)
    /// - [`is_sandbox`](PostV1AccountMeResponseCompaniesItemBuilder::is_sandbox)
    /// - [`status`](PostV1AccountMeResponseCompaniesItemBuilder::status)
    pub fn build(self) -> Result<PostV1AccountMeResponseCompaniesItem, BuildError> {
        Ok(PostV1AccountMeResponseCompaniesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            is_sandbox: self
                .is_sandbox
                .ok_or_else(|| BuildError::missing_field("is_sandbox"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            deleted_at: self.deleted_at,
        })
    }
}
