pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesListLeadsRequest {}

impl TypesListLeadsRequest {
    pub fn builder() -> TypesListLeadsRequestBuilder {
        <TypesListLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesListLeadsRequestBuilder {}

impl TypesListLeadsRequestBuilder {
    /// Consumes the builder and constructs a [`TypesListLeadsRequest`].
    pub fn build(self) -> Result<TypesListLeadsRequest, BuildError> {
        Ok(TypesListLeadsRequest {})
    }
}
