pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeriodsListLedgerRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PeriodsListLedgerRequestSortItemDir>,
}

impl PeriodsListLedgerRequestSortItem {
    pub fn builder() -> PeriodsListLedgerRequestSortItemBuilder {
        <PeriodsListLedgerRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodsListLedgerRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PeriodsListLedgerRequestSortItemDir>,
}

impl PeriodsListLedgerRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PeriodsListLedgerRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodsListLedgerRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PeriodsListLedgerRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PeriodsListLedgerRequestSortItem, BuildError> {
        Ok(PeriodsListLedgerRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
