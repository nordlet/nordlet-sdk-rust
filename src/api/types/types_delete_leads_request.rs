pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesDeleteLeadsRequest {
    #[serde(default)]
    pub id: String,
}

impl TypesDeleteLeadsRequest {
    pub fn builder() -> TypesDeleteLeadsRequestBuilder {
        <TypesDeleteLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesDeleteLeadsRequestBuilder {
    id: Option<String>,
}

impl TypesDeleteLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TypesDeleteLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TypesDeleteLeadsRequestBuilder::id)
    pub fn build(self) -> Result<TypesDeleteLeadsRequest, BuildError> {
        Ok(TypesDeleteLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
