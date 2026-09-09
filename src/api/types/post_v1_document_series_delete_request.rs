pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DocumentSeriesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DocumentSeriesDeleteRequest {
    pub fn builder() -> PostV1DocumentSeriesDeleteRequestBuilder {
        <PostV1DocumentSeriesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1DocumentSeriesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DocumentSeriesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DocumentSeriesDeleteRequest, BuildError> {
        Ok(PostV1DocumentSeriesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
