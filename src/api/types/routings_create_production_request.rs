pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoutingsCreateProductionRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub operations: Vec<RoutingsCreateProductionRequestOperationsItem>,
}

impl RoutingsCreateProductionRequest {
    pub fn builder() -> RoutingsCreateProductionRequestBuilder {
        <RoutingsCreateProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoutingsCreateProductionRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    notes: Option<String>,
    operations: Option<Vec<RoutingsCreateProductionRequestOperationsItem>>,
}

impl RoutingsCreateProductionRequestBuilder {
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

    pub fn operations(mut self, value: Vec<RoutingsCreateProductionRequestOperationsItem>) -> Self {
        self.operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RoutingsCreateProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RoutingsCreateProductionRequestBuilder::code)
    /// - [`name`](RoutingsCreateProductionRequestBuilder::name)
    /// - [`operations`](RoutingsCreateProductionRequestBuilder::operations)
    pub fn build(self) -> Result<RoutingsCreateProductionRequest, BuildError> {
        Ok(RoutingsCreateProductionRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            notes: self.notes,
            operations: self
                .operations
                .ok_or_else(|| BuildError::missing_field("operations"))?,
        })
    }
}
