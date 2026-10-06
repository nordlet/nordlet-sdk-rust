pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesCreateLeadsRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl TypesCreateLeadsRequest {
    pub fn builder() -> TypesCreateLeadsRequestBuilder {
        <TypesCreateLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesCreateLeadsRequestBuilder {
    name: Option<String>,
    is_active: Option<bool>,
}

impl TypesCreateLeadsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TypesCreateLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TypesCreateLeadsRequestBuilder::name)
    pub fn build(self) -> Result<TypesCreateLeadsRequest, BuildError> {
        Ok(TypesCreateLeadsRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_active: self.is_active,
        })
    }
}
