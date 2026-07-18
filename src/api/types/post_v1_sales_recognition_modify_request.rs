pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionModifyRequest {
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    pub approach: PostV1SalesRecognitionModifyRequestApproach,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "newEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_end_date: Option<String>,
    #[serde(rename = "newMilestones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_milestones: Option<Vec<PostV1SalesRecognitionModifyRequestNewMilestonesItem>>,
}

impl PostV1SalesRecognitionModifyRequest {
    pub fn builder() -> PostV1SalesRecognitionModifyRequestBuilder {
        <PostV1SalesRecognitionModifyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionModifyRequestBuilder {
    invoice_line_id: Option<String>,
    approach: Option<PostV1SalesRecognitionModifyRequestApproach>,
    date: Option<String>,
    new_end_date: Option<String>,
    new_milestones: Option<Vec<PostV1SalesRecognitionModifyRequestNewMilestonesItem>>,
}

impl PostV1SalesRecognitionModifyRequestBuilder {
    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn approach(mut self, value: PostV1SalesRecognitionModifyRequestApproach) -> Self {
        self.approach = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn new_end_date(mut self, value: impl Into<String>) -> Self {
        self.new_end_date = Some(value.into());
        self
    }

    pub fn new_milestones(
        mut self,
        value: Vec<PostV1SalesRecognitionModifyRequestNewMilestonesItem>,
    ) -> Self {
        self.new_milestones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionModifyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_line_id`](PostV1SalesRecognitionModifyRequestBuilder::invoice_line_id)
    /// - [`approach`](PostV1SalesRecognitionModifyRequestBuilder::approach)
    pub fn build(self) -> Result<PostV1SalesRecognitionModifyRequest, BuildError> {
        Ok(PostV1SalesRecognitionModifyRequest {
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
