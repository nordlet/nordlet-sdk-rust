pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionComputeSalesRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<NaiveDate>,
}

impl RecognitionComputeSalesRequest {
    pub fn builder() -> RecognitionComputeSalesRequestBuilder {
        <RecognitionComputeSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionComputeSalesRequestBuilder {
    as_of_date: Option<NaiveDate>,
}

impl RecognitionComputeSalesRequestBuilder {
    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionComputeSalesRequest`].
    pub fn build(self) -> Result<RecognitionComputeSalesRequest, BuildError> {
        Ok(RecognitionComputeSalesRequest {
            as_of_date: self.as_of_date,
        })
    }
}
