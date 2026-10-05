pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetCalendarResponse {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub kind: GetCalendarResponseKind,
    #[serde(rename = "ruleKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission: Option<GetCalendarResponseSubmission>,
    #[serde(default)]
    pub submissions: Vec<GetCalendarResponseSubmissionsItem>,
    #[serde(rename = "canSubmit")]
    #[serde(default)]
    pub can_submit: bool,
    #[serde(rename = "canAmend")]
    #[serde(default)]
    pub can_amend: bool,
    #[serde(rename = "canDownload")]
    #[serde(default)]
    pub can_download: bool,
    #[serde(default)]
    pub automated: bool,
}

impl GetCalendarResponse {
    pub fn builder() -> GetCalendarResponseBuilder {
        <GetCalendarResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCalendarResponseBuilder {
    key: Option<String>,
    id: Option<String>,
    kind: Option<GetCalendarResponseKind>,
    rule_key: Option<String>,
    period: Option<String>,
    title: Option<String>,
    due_date: Option<NaiveDate>,
    notes: Option<String>,
    done: Option<bool>,
    href: Option<String>,
    submission: Option<GetCalendarResponseSubmission>,
    submissions: Option<Vec<GetCalendarResponseSubmissionsItem>>,
    can_submit: Option<bool>,
    can_amend: Option<bool>,
    can_download: Option<bool>,
    automated: Option<bool>,
}

impl GetCalendarResponseBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: GetCalendarResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn done(mut self, value: bool) -> Self {
        self.done = Some(value);
        self
    }

    pub fn href(mut self, value: impl Into<String>) -> Self {
        self.href = Some(value.into());
        self
    }

    pub fn submission(mut self, value: GetCalendarResponseSubmission) -> Self {
        self.submission = Some(value);
        self
    }

    pub fn submissions(mut self, value: Vec<GetCalendarResponseSubmissionsItem>) -> Self {
        self.submissions = Some(value);
        self
    }

    pub fn can_submit(mut self, value: bool) -> Self {
        self.can_submit = Some(value);
        self
    }

    pub fn can_amend(mut self, value: bool) -> Self {
        self.can_amend = Some(value);
        self
    }

    pub fn can_download(mut self, value: bool) -> Self {
        self.can_download = Some(value);
        self
    }

    pub fn automated(mut self, value: bool) -> Self {
        self.automated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCalendarResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](GetCalendarResponseBuilder::key)
    /// - [`kind`](GetCalendarResponseBuilder::kind)
    /// - [`title`](GetCalendarResponseBuilder::title)
    /// - [`due_date`](GetCalendarResponseBuilder::due_date)
    /// - [`done`](GetCalendarResponseBuilder::done)
    /// - [`submissions`](GetCalendarResponseBuilder::submissions)
    /// - [`can_submit`](GetCalendarResponseBuilder::can_submit)
    /// - [`can_amend`](GetCalendarResponseBuilder::can_amend)
    /// - [`can_download`](GetCalendarResponseBuilder::can_download)
    /// - [`automated`](GetCalendarResponseBuilder::automated)
    pub fn build(self) -> Result<GetCalendarResponse, BuildError> {
        Ok(GetCalendarResponse {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            id: self.id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rule_key: self.rule_key,
            period: self.period,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            due_date: self
                .due_date
                .ok_or_else(|| BuildError::missing_field("due_date"))?,
            notes: self.notes,
            done: self.done.ok_or_else(|| BuildError::missing_field("done"))?,
            href: self.href,
            submission: self.submission,
            submissions: self
                .submissions
                .ok_or_else(|| BuildError::missing_field("submissions"))?,
            can_submit: self
                .can_submit
                .ok_or_else(|| BuildError::missing_field("can_submit"))?,
            can_amend: self
                .can_amend
                .ok_or_else(|| BuildError::missing_field("can_amend"))?,
            can_download: self
                .can_download
                .ok_or_else(|| BuildError::missing_field("can_download"))?,
            automated: self
                .automated
                .ok_or_else(|| BuildError::missing_field("automated"))?,
        })
    }
}
