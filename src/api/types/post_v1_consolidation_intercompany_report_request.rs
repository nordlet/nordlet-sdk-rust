pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
}

impl PostV1ConsolidationIntercompanyReportRequest {
    pub fn builder() -> PostV1ConsolidationIntercompanyReportRequestBuilder {
        <PostV1ConsolidationIntercompanyReportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportRequestBuilder {
    group_id: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
}

impl PostV1ConsolidationIntercompanyReportRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationIntercompanyReportRequestBuilder::group_id)
    /// - [`from_date`](PostV1ConsolidationIntercompanyReportRequestBuilder::from_date)
    /// - [`to_date`](PostV1ConsolidationIntercompanyReportRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyReportRequest, BuildError> {
        Ok(PostV1ConsolidationIntercompanyReportRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
