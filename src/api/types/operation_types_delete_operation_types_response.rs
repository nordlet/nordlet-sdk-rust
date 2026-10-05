pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteOperationTypesResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl DeleteOperationTypesResponse {
    pub fn builder() -> DeleteOperationTypesResponseBuilder {
        <DeleteOperationTypesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteOperationTypesResponseBuilder {
    deleted: Option<bool>,
}

impl DeleteOperationTypesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteOperationTypesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteOperationTypesResponseBuilder::deleted)
    pub fn build(self) -> Result<DeleteOperationTypesResponse, BuildError> {
        Ok(DeleteOperationTypesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
