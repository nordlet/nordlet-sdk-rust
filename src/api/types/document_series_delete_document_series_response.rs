pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDocumentSeriesResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl DeleteDocumentSeriesResponse {
    pub fn builder() -> DeleteDocumentSeriesResponseBuilder {
        <DeleteDocumentSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDocumentSeriesResponseBuilder {
    deleted: Option<bool>,
}

impl DeleteDocumentSeriesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteDocumentSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteDocumentSeriesResponseBuilder::deleted)
    pub fn build(self) -> Result<DeleteDocumentSeriesResponse, BuildError> {
        Ok(DeleteDocumentSeriesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
