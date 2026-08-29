pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesBillRequest {
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "dateFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "hourlyRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "vatClassifierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_classifier_code: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<PostV1ProjectsTimeEntriesBillRequestGroupBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProjectsTimeEntriesBillRequest {
    pub fn builder() -> PostV1ProjectsTimeEntriesBillRequestBuilder {
        <PostV1ProjectsTimeEntriesBillRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesBillRequestBuilder {
    project_id: Option<String>,
    partner_id: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    item_id: Option<String>,
    hourly_rate: Option<String>,
    vat_rate_percent: Option<String>,
    vat_classifier_code: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    group_by: Option<PostV1ProjectsTimeEntriesBillRequestGroupBy>,
    notes: Option<String>,
}

impl PostV1ProjectsTimeEntriesBillRequestBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn hourly_rate(mut self, value: impl Into<String>) -> Self {
        self.hourly_rate = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn vat_classifier_code(mut self, value: impl Into<String>) -> Self {
        self.vat_classifier_code = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    pub fn group_by(mut self, value: PostV1ProjectsTimeEntriesBillRequestGroupBy) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesBillRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](PostV1ProjectsTimeEntriesBillRequestBuilder::project_id)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesBillRequest, BuildError> {
        Ok(PostV1ProjectsTimeEntriesBillRequest {
            project_id: self
                .project_id
                .ok_or_else(|| BuildError::missing_field("project_id"))?,
            partner_id: self.partner_id,
            date_from: self.date_from,
            date_to: self.date_to,
            item_id: self.item_id,
            hourly_rate: self.hourly_rate,
            vat_rate_percent: self.vat_rate_percent,
            vat_classifier_code: self.vat_classifier_code,
            issue_date: self.issue_date,
            due_date: self.due_date,
            group_by: self.group_by,
            notes: self.notes,
        })
    }
}
