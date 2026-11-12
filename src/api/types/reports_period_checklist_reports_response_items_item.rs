pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeriodChecklistReportsResponseItemsItem {
    pub code: PeriodChecklistReportsResponseItemsItemCode,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub ids: Vec<String>,
}

impl PeriodChecklistReportsResponseItemsItem {
    pub fn builder() -> PeriodChecklistReportsResponseItemsItemBuilder {
        <PeriodChecklistReportsResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodChecklistReportsResponseItemsItemBuilder {
    code: Option<PeriodChecklistReportsResponseItemsItemCode>,
    count: Option<i64>,
    message: Option<String>,
    ids: Option<Vec<String>>,
}

impl PeriodChecklistReportsResponseItemsItemBuilder {
    pub fn code(mut self, value: PeriodChecklistReportsResponseItemsItemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn ids(mut self, value: Vec<String>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodChecklistReportsResponseItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PeriodChecklistReportsResponseItemsItemBuilder::code)
    /// - [`count`](PeriodChecklistReportsResponseItemsItemBuilder::count)
    /// - [`message`](PeriodChecklistReportsResponseItemsItemBuilder::message)
    /// - [`ids`](PeriodChecklistReportsResponseItemsItemBuilder::ids)
    pub fn build(self) -> Result<PeriodChecklistReportsResponseItemsItem, BuildError> {
        Ok(PeriodChecklistReportsResponseItemsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
        })
    }
}
