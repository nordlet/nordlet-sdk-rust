pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesUpdateSalesRequestLinesItemRecognition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<InvoicesUpdateSalesRequestLinesItemRecognitionMethod>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem>>,
}

impl InvoicesUpdateSalesRequestLinesItemRecognition {
    pub fn builder() -> InvoicesUpdateSalesRequestLinesItemRecognitionBuilder {
        <InvoicesUpdateSalesRequestLinesItemRecognitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUpdateSalesRequestLinesItemRecognitionBuilder {
    method: Option<InvoicesUpdateSalesRequestLinesItemRecognitionMethod>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    milestones: Option<Vec<InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem>>,
}

impl InvoicesUpdateSalesRequestLinesItemRecognitionBuilder {
    pub fn method(mut self, value: InvoicesUpdateSalesRequestLinesItemRecognitionMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn milestones(
        mut self,
        value: Vec<InvoicesUpdateSalesRequestLinesItemRecognitionMilestonesItem>,
    ) -> Self {
        self.milestones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesUpdateSalesRequestLinesItemRecognition`].
    pub fn build(self) -> Result<InvoicesUpdateSalesRequestLinesItemRecognition, BuildError> {
        Ok(InvoicesUpdateSalesRequestLinesItemRecognition {
            method: self.method,
            start_date: self.start_date,
            end_date: self.end_date,
            milestones: self.milestones,
        })
    }
}
