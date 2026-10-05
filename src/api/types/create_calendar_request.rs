pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCalendarRequest {
    #[serde(default)]
    pub title: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

impl CreateCalendarRequest {
    pub fn builder() -> CreateCalendarRequestBuilder {
        <CreateCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCalendarRequestBuilder {
    title: Option<String>,
    due_date: Option<NaiveDate>,
    notes: Option<String>,
    done: Option<bool>,
}

impl CreateCalendarRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreateCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateCalendarRequestBuilder::title)
    /// - [`due_date`](CreateCalendarRequestBuilder::due_date)
    pub fn build(self) -> Result<CreateCalendarRequest, BuildError> {
        Ok(CreateCalendarRequest {
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
