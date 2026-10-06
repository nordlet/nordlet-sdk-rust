pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesUpdateLeadsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl TypesUpdateLeadsRequest {
    pub fn builder() -> TypesUpdateLeadsRequestBuilder {
        <TypesUpdateLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesUpdateLeadsRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
}

impl TypesUpdateLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`TypesUpdateLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TypesUpdateLeadsRequestBuilder::id)
    pub fn build(self) -> Result<TypesUpdateLeadsRequest, BuildError> {
        Ok(TypesUpdateLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            is_active: self.is_active,
        })
    }
}
