pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarUpdateRequest {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

impl PostV1CalendarUpdateRequest {
    pub fn builder() -> PostV1CalendarUpdateRequestBuilder {
        <PostV1CalendarUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarUpdateRequestBuilder {
    key: Option<String>,
    title: Option<String>,
    due_date: Option<String>,
    notes: Option<String>,
    done: Option<bool>,
}

impl PostV1CalendarUpdateRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1CalendarUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarUpdateRequestBuilder::key)
    pub fn build(self) -> Result<PostV1CalendarUpdateRequest, BuildError> {
        Ok(PostV1CalendarUpdateRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            title: self.title,
            due_date: self.due_date,
            notes: self.notes,
            done: self.done,
        })
    }
}
