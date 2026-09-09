pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarCreateRequest {
    #[serde(default)]
    pub title: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

impl PostV1CalendarCreateRequest {
    pub fn builder() -> PostV1CalendarCreateRequestBuilder {
        <PostV1CalendarCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarCreateRequestBuilder {
    title: Option<String>,
    due_date: Option<String>,
    notes: Option<String>,
    done: Option<bool>,
}

impl PostV1CalendarCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1CalendarCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](PostV1CalendarCreateRequestBuilder::title)
    /// - [`due_date`](PostV1CalendarCreateRequestBuilder::due_date)
    pub fn build(self) -> Result<PostV1CalendarCreateRequest, BuildError> {
        Ok(PostV1CalendarCreateRequest {
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            due_date: self
                .due_date
                .ok_or_else(|| BuildError::missing_field("due_date"))?,
            notes: self.notes,
            done: self.done,
        })
    }
}
