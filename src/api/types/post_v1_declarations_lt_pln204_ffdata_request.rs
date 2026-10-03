pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtPln204FfdataRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsLtPln204FfdataRequest {
    pub fn builder() -> PostV1DeclarationsLtPln204FfdataRequestBuilder {
        <PostV1DeclarationsLtPln204FfdataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204FfdataRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsLtPln204FfdataRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204FfdataRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtPln204FfdataRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204FfdataRequest, BuildError> {
        Ok(PostV1DeclarationsLtPln204FfdataRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
