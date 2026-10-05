pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCalendarRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<NaiveDate>,
    #[serde(rename = "includeDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_done: Option<bool>,
}

impl ListCalendarRequest {
    pub fn builder() -> ListCalendarRequestBuilder {
        <ListCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalendarRequestBuilder {
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    include_done: Option<bool>,
}

impl ListCalendarRequestBuilder {
    pub fn from(mut self, value: NaiveDate) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: NaiveDate) -> Self {
        self.to = Some(value);
        self
    }

    pub fn include_done(mut self, value: bool) -> Self {
        self.include_done = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCalendarRequest`].
    pub fn build(self) -> Result<ListCalendarRequest, BuildError> {
        Ok(ListCalendarRequest {
            from: self.from,
            to: self.to,
            include_done: self.include_done,
        })
    }
}
