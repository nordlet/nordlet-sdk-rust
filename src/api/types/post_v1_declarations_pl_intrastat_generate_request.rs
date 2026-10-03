pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlIntrastatGenerateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub flow: PostV1DeclarationsPlIntrastatGenerateRequestFlow,
    #[serde(rename = "transactionNature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_nature: Option<String>,
}

impl PostV1DeclarationsPlIntrastatGenerateRequest {
    pub fn builder() -> PostV1DeclarationsPlIntrastatGenerateRequestBuilder {
        <PostV1DeclarationsPlIntrastatGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlIntrastatGenerateRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    flow: Option<PostV1DeclarationsPlIntrastatGenerateRequestFlow>,
    transaction_nature: Option<String>,
}

impl PostV1DeclarationsPlIntrastatGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn flow(mut self, value: PostV1DeclarationsPlIntrastatGenerateRequestFlow) -> Self {
        self.flow = Some(value);
        self
    }

    pub fn transaction_nature(mut self, value: impl Into<String>) -> Self {
        self.transaction_nature = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlIntrastatGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlIntrastatGenerateRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsPlIntrastatGenerateRequestBuilder::month)
    /// - [`flow`](PostV1DeclarationsPlIntrastatGenerateRequestBuilder::flow)
    pub fn build(self) -> Result<PostV1DeclarationsPlIntrastatGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsPlIntrastatGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            flow: self.flow.ok_or_else(|| BuildError::missing_field("flow"))?,
            transaction_nature: self.transaction_nature,
        })
    }
}
