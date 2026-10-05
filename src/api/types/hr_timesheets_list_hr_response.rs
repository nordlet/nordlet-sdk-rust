pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsListHrResponse {
    #[serde(default)]
    pub rows: Vec<TimesheetsListHrResponseRowsItem>,
}

impl TimesheetsListHrResponse {
    pub fn builder() -> TimesheetsListHrResponseBuilder {
        <TimesheetsListHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsListHrResponseBuilder {
    rows: Option<Vec<TimesheetsListHrResponseRowsItem>>,
}

impl TimesheetsListHrResponseBuilder {
    pub fn rows(mut self, value: Vec<TimesheetsListHrResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsListHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TimesheetsListHrResponseBuilder::rows)
    pub fn build(self) -> Result<TimesheetsListHrResponse, BuildError> {
        Ok(TimesheetsListHrResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
