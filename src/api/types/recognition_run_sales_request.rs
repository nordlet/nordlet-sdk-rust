pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionRunSalesRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<NaiveDate>,
    #[serde(rename = "postingDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posting_date: Option<NaiveDate>,
    #[serde(rename = "scheduleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_ids: Option<Vec<String>>,
}

impl RecognitionRunSalesRequest {
    pub fn builder() -> RecognitionRunSalesRequestBuilder {
        <RecognitionRunSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionRunSalesRequestBuilder {
    as_of_date: Option<NaiveDate>,
    posting_date: Option<NaiveDate>,
    schedule_ids: Option<Vec<String>>,
}

impl RecognitionRunSalesRequestBuilder {
    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    pub fn posting_date(mut self, value: NaiveDate) -> Self {
        self.posting_date = Some(value);
        self
    }

    pub fn schedule_ids(mut self, value: Vec<String>) -> Self {
        self.schedule_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionRunSalesRequest`].
    pub fn build(self) -> Result<RecognitionRunSalesRequest, BuildError> {
        Ok(RecognitionRunSalesRequest {
            as_of_date: self.as_of_date,
            posting_date: self.posting_date,
            schedule_ids: self.schedule_ids,
        })
    }
}
