pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatSummaryRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<PostV1ReportsVatSummaryRequestSide>,
}

impl PostV1ReportsVatSummaryRequest {
    pub fn builder() -> PostV1ReportsVatSummaryRequestBuilder {
        <PostV1ReportsVatSummaryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatSummaryRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    side: Option<PostV1ReportsVatSummaryRequestSide>,
}

impl PostV1ReportsVatSummaryRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn side(mut self, value: PostV1ReportsVatSummaryRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsVatSummaryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsVatSummaryRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsVatSummaryRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsVatSummaryRequest, BuildError> {
        Ok(PostV1ReportsVatSummaryRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            side: self.side,
        })
    }
}
