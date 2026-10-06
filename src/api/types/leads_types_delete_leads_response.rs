pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesDeleteLeadsResponse {
    #[serde(default)]
    pub id: String,
}

impl TypesDeleteLeadsResponse {
    pub fn builder() -> TypesDeleteLeadsResponseBuilder {
        <TypesDeleteLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesDeleteLeadsResponseBuilder {
    id: Option<String>,
}

impl TypesDeleteLeadsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TypesDeleteLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TypesDeleteLeadsResponseBuilder::id)
    pub fn build(self) -> Result<TypesDeleteLeadsResponse, BuildError> {
        Ok(TypesDeleteLeadsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
