pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1CalendarListResponseRowsItem {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub kind: PostV1CalendarListResponseRowsItemKind,
    #[serde(rename = "ruleKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl PostV1CalendarListResponseRowsItem {
    pub fn builder() -> PostV1CalendarListResponseRowsItemBuilder {
        <PostV1CalendarListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarListResponseRowsItemBuilder {
    key: Option<String>,
    id: Option<String>,
    kind: Option<PostV1CalendarListResponseRowsItemKind>,
    rule_key: Option<String>,
    period: Option<String>,
    title: Option<String>,
    due_date: Option<String>,
    notes: Option<String>,
    done: Option<bool>,
    href: Option<String>,
}

impl PostV1CalendarListResponseRowsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1CalendarListResponseRowsItemKind) -> Self {
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

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1CalendarListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarListResponseRowsItemBuilder::key)
    /// - [`kind`](PostV1CalendarListResponseRowsItemBuilder::kind)
    /// - [`title`](PostV1CalendarListResponseRowsItemBuilder::title)
    /// - [`due_date`](PostV1CalendarListResponseRowsItemBuilder::due_date)
    /// - [`done`](PostV1CalendarListResponseRowsItemBuilder::done)
    pub fn build(self) -> Result<PostV1CalendarListResponseRowsItem, BuildError> {
        Ok(PostV1CalendarListResponseRowsItem {
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
        })
    }
}
