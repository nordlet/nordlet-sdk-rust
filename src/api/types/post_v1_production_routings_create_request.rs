pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionRoutingsCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub operations: Vec<PostV1ProductionRoutingsCreateRequestOperationsItem>,
}

impl PostV1ProductionRoutingsCreateRequest {
    pub fn builder() -> PostV1ProductionRoutingsCreateRequestBuilder {
        <PostV1ProductionRoutingsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionRoutingsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    notes: Option<String>,
    operations: Option<Vec<PostV1ProductionRoutingsCreateRequestOperationsItem>>,
}

impl PostV1ProductionRoutingsCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn operations(
        mut self,
        value: Vec<PostV1ProductionRoutingsCreateRequestOperationsItem>,
    ) -> Self {
        self.operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionRoutingsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ProductionRoutingsCreateRequestBuilder::code)
    /// - [`name`](PostV1ProductionRoutingsCreateRequestBuilder::name)
    /// - [`operations`](PostV1ProductionRoutingsCreateRequestBuilder::operations)
    pub fn build(self) -> Result<PostV1ProductionRoutingsCreateRequest, BuildError> {
        Ok(PostV1ProductionRoutingsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            notes: self.notes,
            operations: self
                .operations
                .ok_or_else(|| BuildError::missing_field("operations"))?,
        })
    }
}
