pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlZusDraPdfRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsPlZusDraPdfRequest {
    pub fn builder() -> PostV1DeclarationsPlZusDraPdfRequestBuilder {
        <PostV1DeclarationsPlZusDraPdfRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlZusDraPdfRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsPlZusDraPdfRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlZusDraPdfRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlZusDraPdfRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsPlZusDraPdfRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsPlZusDraPdfRequest, BuildError> {
        Ok(PostV1DeclarationsPlZusDraPdfRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
