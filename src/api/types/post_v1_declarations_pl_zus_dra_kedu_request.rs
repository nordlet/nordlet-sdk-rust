pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlZusDraKeduRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsPlZusDraKeduRequest {
    pub fn builder() -> PostV1DeclarationsPlZusDraKeduRequestBuilder {
        <PostV1DeclarationsPlZusDraKeduRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlZusDraKeduRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsPlZusDraKeduRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlZusDraKeduRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlZusDraKeduRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsPlZusDraKeduRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsPlZusDraKeduRequest, BuildError> {
        Ok(PostV1DeclarationsPlZusDraKeduRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
