pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteFilesResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl DeleteFilesResponse {
    pub fn builder() -> DeleteFilesResponseBuilder {
        <DeleteFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteFilesResponseBuilder {
    deleted: Option<bool>,
}

impl DeleteFilesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteFilesResponseBuilder::deleted)
    pub fn build(self) -> Result<DeleteFilesResponse, BuildError> {
        Ok(DeleteFilesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
