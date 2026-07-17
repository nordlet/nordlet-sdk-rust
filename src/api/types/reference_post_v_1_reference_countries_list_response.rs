pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCountriesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceCountriesListResponseRowsItem>,
}

impl PostV1ReferenceCountriesListResponse {
    pub fn builder() -> PostV1ReferenceCountriesListResponseBuilder {
        <PostV1ReferenceCountriesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCountriesListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceCountriesListResponseRowsItem>>,
}

impl PostV1ReferenceCountriesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceCountriesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCountriesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceCountriesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceCountriesListResponse, BuildError> {
        Ok(PostV1ReferenceCountriesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
