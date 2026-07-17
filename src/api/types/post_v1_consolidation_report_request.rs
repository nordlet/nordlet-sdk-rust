pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<PostV1ConsolidationReportRequestCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eliminations: Option<Vec<PostV1ConsolidationReportRequestEliminationsItem>>,
}

impl PostV1ConsolidationReportRequest {
    pub fn builder() -> PostV1ConsolidationReportRequestBuilder {
        <PostV1ConsolidationReportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportRequestBuilder {
    group_id: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    category: Option<PostV1ConsolidationReportRequestCategory>,
    eliminations: Option<Vec<PostV1ConsolidationReportRequestEliminationsItem>>,
}

impl PostV1ConsolidationReportRequestBuilder {
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

    pub fn category(mut self, value: PostV1ConsolidationReportRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn eliminations(
        mut self,
        value: Vec<PostV1ConsolidationReportRequestEliminationsItem>,
    ) -> Self {
        self.eliminations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationReportRequestBuilder::group_id)
    /// - [`from_date`](PostV1ConsolidationReportRequestBuilder::from_date)
    /// - [`to_date`](PostV1ConsolidationReportRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ConsolidationReportRequest, BuildError> {
        Ok(PostV1ConsolidationReportRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            category: self.category,
            eliminations: self.eliminations,
        })
    }
}
