pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceIntrastatThresholdsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceIntrastatThresholdsListResponseRowsItem>,
}

impl PostV1ReferenceIntrastatThresholdsListResponse {
    pub fn builder() -> PostV1ReferenceIntrastatThresholdsListResponseBuilder {
        <PostV1ReferenceIntrastatThresholdsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceIntrastatThresholdsListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceIntrastatThresholdsListResponseRowsItem>>,
}

impl PostV1ReferenceIntrastatThresholdsListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1ReferenceIntrastatThresholdsListResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceIntrastatThresholdsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceIntrastatThresholdsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceIntrastatThresholdsListResponse, BuildError> {
        Ok(PostV1ReferenceIntrastatThresholdsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
