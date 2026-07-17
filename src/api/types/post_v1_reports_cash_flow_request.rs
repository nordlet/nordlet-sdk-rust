pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCashFlowRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
}

impl PostV1ReportsCashFlowRequest {
    pub fn builder() -> PostV1ReportsCashFlowRequestBuilder {
        <PostV1ReportsCashFlowRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCashFlowRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
}

impl PostV1ReportsCashFlowRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCashFlowRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsCashFlowRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsCashFlowRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsCashFlowRequest, BuildError> {
        Ok(PostV1ReportsCashFlowRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
