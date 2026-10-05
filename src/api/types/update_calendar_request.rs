pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCalendarRequest {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

impl UpdateCalendarRequest {
    pub fn builder() -> UpdateCalendarRequestBuilder {
        <UpdateCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarRequestBuilder {
    key: Option<String>,
    title: Option<String>,
    due_date: Option<NaiveDate>,
    notes: Option<String>,
    done: Option<bool>,
}

impl UpdateCalendarRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](UpdateCalendarRequestBuilder::key)
    pub fn build(self) -> Result<UpdateCalendarRequest, BuildError> {
        Ok(UpdateCalendarRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            title: self.title,
            due_date: self.due_date,
            notes: self.notes,
            done: self.done,
        })
    }
}
