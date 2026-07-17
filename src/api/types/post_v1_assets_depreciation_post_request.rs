pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsDepreciationPostRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1AssetsDepreciationPostRequest {
    pub fn builder() -> PostV1AssetsDepreciationPostRequestBuilder {
        <PostV1AssetsDepreciationPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsDepreciationPostRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1AssetsDepreciationPostRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsDepreciationPostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1AssetsDepreciationPostRequestBuilder::year)
    /// - [`month`](PostV1AssetsDepreciationPostRequestBuilder::month)
    pub fn build(self) -> Result<PostV1AssetsDepreciationPostRequest, BuildError> {
        Ok(PostV1AssetsDepreciationPostRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
