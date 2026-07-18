pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionRunRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<String>,
    #[serde(rename = "postingDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posting_date: Option<String>,
    #[serde(rename = "scheduleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_ids: Option<Vec<String>>,
}

impl PostV1SalesRecognitionRunRequest {
    pub fn builder() -> PostV1SalesRecognitionRunRequestBuilder {
        <PostV1SalesRecognitionRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionRunRequestBuilder {
    as_of_date: Option<String>,
    posting_date: Option<String>,
    schedule_ids: Option<Vec<String>>,
}

impl PostV1SalesRecognitionRunRequestBuilder {
    pub fn as_of_date(mut self, value: impl Into<String>) -> Self {
        self.as_of_date = Some(value.into());
        self
    }

    pub fn posting_date(mut self, value: impl Into<String>) -> Self {
        self.posting_date = Some(value.into());
        self
    }

    pub fn schedule_ids(mut self, value: Vec<String>) -> Self {
        self.schedule_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionRunRequest`].
    pub fn build(self) -> Result<PostV1SalesRecognitionRunRequest, BuildError> {
        Ok(PostV1SalesRecognitionRunRequest {
            as_of_date: self.as_of_date,
            posting_date: self.posting_date,
            schedule_ids: self.schedule_ids,
        })
    }
}
