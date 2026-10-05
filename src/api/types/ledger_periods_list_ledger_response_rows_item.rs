pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeriodsListLedgerResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub status: PeriodsListLedgerResponseRowsItemStatus,
}

impl PeriodsListLedgerResponseRowsItem {
    pub fn builder() -> PeriodsListLedgerResponseRowsItemBuilder {
        <PeriodsListLedgerResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodsListLedgerResponseRowsItemBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    status: Option<PeriodsListLedgerResponseRowsItemStatus>,
}

impl PeriodsListLedgerResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn status(mut self, value: PeriodsListLedgerResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodsListLedgerResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PeriodsListLedgerResponseRowsItemBuilder::id)
    /// - [`year`](PeriodsListLedgerResponseRowsItemBuilder::year)
    /// - [`month`](PeriodsListLedgerResponseRowsItemBuilder::month)
    /// - [`status`](PeriodsListLedgerResponseRowsItemBuilder::status)
    pub fn build(self) -> Result<PeriodsListLedgerResponseRowsItem, BuildError> {
        Ok(PeriodsListLedgerResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
