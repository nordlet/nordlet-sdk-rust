pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionRoutingsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub operations: Vec<PostV1ProductionRoutingsGetResponseOperationsItem>,
}

impl PostV1ProductionRoutingsGetResponse {
    pub fn builder() -> PostV1ProductionRoutingsGetResponseBuilder {
        <PostV1ProductionRoutingsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionRoutingsGetResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
    created_at: Option<String>,
    operations: Option<Vec<PostV1ProductionRoutingsGetResponseOperationsItem>>,
}

impl PostV1ProductionRoutingsGetResponseBuilder {
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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn operations(
        mut self,
        value: Vec<PostV1ProductionRoutingsGetResponseOperationsItem>,
    ) -> Self {
        self.operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionRoutingsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionRoutingsGetResponseBuilder::id)
    /// - [`code`](PostV1ProductionRoutingsGetResponseBuilder::code)
    /// - [`name`](PostV1ProductionRoutingsGetResponseBuilder::name)
    /// - [`is_active`](PostV1ProductionRoutingsGetResponseBuilder::is_active)
    /// - [`created_at`](PostV1ProductionRoutingsGetResponseBuilder::created_at)
    /// - [`operations`](PostV1ProductionRoutingsGetResponseBuilder::operations)
    pub fn build(self) -> Result<PostV1ProductionRoutingsGetResponse, BuildError> {
        Ok(PostV1ProductionRoutingsGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            operations: self
                .operations
                .ok_or_else(|| BuildError::missing_field("operations"))?,
        })
    }
}
