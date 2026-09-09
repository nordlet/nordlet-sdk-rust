pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DocumentSeriesDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1DocumentSeriesDeleteResponse {
    pub fn builder() -> PostV1DocumentSeriesDeleteResponseBuilder {
        <PostV1DocumentSeriesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1DocumentSeriesDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1DocumentSeriesDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1DocumentSeriesDeleteResponse, BuildError> {
        Ok(PostV1DocumentSeriesDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
