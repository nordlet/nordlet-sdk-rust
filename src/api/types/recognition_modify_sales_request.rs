pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecognitionModifySalesRequest {
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    pub approach: RecognitionModifySalesRequestApproach,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "newEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_end_date: Option<NaiveDate>,
    #[serde(rename = "newMilestones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_milestones: Option<Vec<RecognitionModifySalesRequestNewMilestonesItem>>,
}

impl RecognitionModifySalesRequest {
    pub fn builder() -> RecognitionModifySalesRequestBuilder {
        <RecognitionModifySalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionModifySalesRequestBuilder {
    invoice_line_id: Option<String>,
    approach: Option<RecognitionModifySalesRequestApproach>,
    date: Option<NaiveDate>,
    new_end_date: Option<NaiveDate>,
    new_milestones: Option<Vec<RecognitionModifySalesRequestNewMilestonesItem>>,
}

impl RecognitionModifySalesRequestBuilder {
    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn approach(mut self, value: RecognitionModifySalesRequestApproach) -> Self {
        self.approach = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn new_end_date(mut self, value: NaiveDate) -> Self {
        self.new_end_date = Some(value);
        self
    }

    pub fn new_milestones(
        mut self,
        value: Vec<RecognitionModifySalesRequestNewMilestonesItem>,
    ) -> Self {
        self.new_milestones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionModifySalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_line_id`](RecognitionModifySalesRequestBuilder::invoice_line_id)
    /// - [`approach`](RecognitionModifySalesRequestBuilder::approach)
    pub fn build(self) -> Result<RecognitionModifySalesRequest, BuildError> {
        Ok(RecognitionModifySalesRequest {
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            approach: self
                .approach
                .ok_or_else(|| BuildError::missing_field("approach"))?,
            date: self.date,
            new_end_date: self.new_end_date,
            new_milestones: self.new_milestones,
        })
    }
}
