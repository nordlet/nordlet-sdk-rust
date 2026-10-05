pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilesListLeadsResponse {
    #[serde(default)]
    pub rows: Vec<FilesListLeadsResponseRowsItem>,
}

impl FilesListLeadsResponse {
    pub fn builder() -> FilesListLeadsResponseBuilder {
        <FilesListLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilesListLeadsResponseBuilder {
    rows: Option<Vec<FilesListLeadsResponseRowsItem>>,
}

impl FilesListLeadsResponseBuilder {
    pub fn rows(mut self, value: Vec<FilesListLeadsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilesListLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](FilesListLeadsResponseBuilder::rows)
    pub fn build(self) -> Result<FilesListLeadsResponse, BuildError> {
        Ok(FilesListLeadsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
