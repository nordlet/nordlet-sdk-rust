pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIsafGenerateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<PostV1DeclarationsLtIsafGenerateRequestDataType>,
}

impl PostV1DeclarationsLtIsafGenerateRequest {
    pub fn builder() -> PostV1DeclarationsLtIsafGenerateRequestBuilder {
        <PostV1DeclarationsLtIsafGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIsafGenerateRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    data_type: Option<PostV1DeclarationsLtIsafGenerateRequestDataType>,
}

impl PostV1DeclarationsLtIsafGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn data_type(mut self, value: PostV1DeclarationsLtIsafGenerateRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIsafGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtIsafGenerateRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsLtIsafGenerateRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtIsafGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsLtIsafGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            data_type: self.data_type,
        })
    }
}
