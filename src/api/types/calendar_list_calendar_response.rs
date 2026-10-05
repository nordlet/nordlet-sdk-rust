pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCalendarResponse {
    #[serde(default)]
    pub rows: Vec<ListCalendarResponseRowsItem>,
}

impl ListCalendarResponse {
    pub fn builder() -> ListCalendarResponseBuilder {
        <ListCalendarResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalendarResponseBuilder {
    rows: Option<Vec<ListCalendarResponseRowsItem>>,
}

impl ListCalendarResponseBuilder {
    pub fn rows(mut self, value: Vec<ListCalendarResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCalendarResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ListCalendarResponseBuilder::rows)
    pub fn build(self) -> Result<ListCalendarResponse, BuildError> {
        Ok(ListCalendarResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
