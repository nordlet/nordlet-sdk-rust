pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilesListPartnersResponse {
    #[serde(default)]
    pub rows: Vec<FilesListPartnersResponseRowsItem>,
}

impl FilesListPartnersResponse {
    pub fn builder() -> FilesListPartnersResponseBuilder {
        <FilesListPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilesListPartnersResponseBuilder {
    rows: Option<Vec<FilesListPartnersResponseRowsItem>>,
}

impl FilesListPartnersResponseBuilder {
    pub fn rows(mut self, value: Vec<FilesListPartnersResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilesListPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](FilesListPartnersResponseBuilder::rows)
    pub fn build(self) -> Result<FilesListPartnersResponse, BuildError> {
        Ok(FilesListPartnersResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
