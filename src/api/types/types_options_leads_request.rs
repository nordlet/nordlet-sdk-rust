pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesOptionsLeadsRequest {}

impl TypesOptionsLeadsRequest {
    pub fn builder() -> TypesOptionsLeadsRequestBuilder {
        <TypesOptionsLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesOptionsLeadsRequestBuilder {}

impl TypesOptionsLeadsRequestBuilder {
    /// Consumes the builder and constructs a [`TypesOptionsLeadsRequest`].
    pub fn build(self) -> Result<TypesOptionsLeadsRequest, BuildError> {
        Ok(TypesOptionsLeadsRequest {})
    }
}
