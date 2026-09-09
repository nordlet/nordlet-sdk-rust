pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DocumentSeriesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DocumentSeriesGetRequest {
    pub fn builder() -> PostV1DocumentSeriesGetRequestBuilder {
        <PostV1DocumentSeriesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1DocumentSeriesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DocumentSeriesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DocumentSeriesGetRequest, BuildError> {
        Ok(PostV1DocumentSeriesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
