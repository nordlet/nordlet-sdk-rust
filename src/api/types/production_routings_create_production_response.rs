pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoutingsCreateProductionResponse {
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub operations: Vec<RoutingsCreateProductionResponseOperationsItem>,
}

impl RoutingsCreateProductionResponse {
    pub fn builder() -> RoutingsCreateProductionResponseBuilder {
        <RoutingsCreateProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoutingsCreateProductionResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    operations: Option<Vec<RoutingsCreateProductionResponseOperationsItem>>,
}

impl RoutingsCreateProductionResponseBuilder {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn operations(
        mut self,
        value: Vec<RoutingsCreateProductionResponseOperationsItem>,
    ) -> Self {
        self.operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RoutingsCreateProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RoutingsCreateProductionResponseBuilder::id)
    /// - [`code`](RoutingsCreateProductionResponseBuilder::code)
    /// - [`name`](RoutingsCreateProductionResponseBuilder::name)
    /// - [`is_active`](RoutingsCreateProductionResponseBuilder::is_active)
    /// - [`created_at`](RoutingsCreateProductionResponseBuilder::created_at)
    /// - [`operations`](RoutingsCreateProductionResponseBuilder::operations)
    pub fn build(self) -> Result<RoutingsCreateProductionResponse, BuildError> {
        Ok(RoutingsCreateProductionResponse {
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
