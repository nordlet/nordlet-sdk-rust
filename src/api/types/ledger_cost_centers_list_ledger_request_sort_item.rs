pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCentersListLedgerRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<CostCentersListLedgerRequestSortItemDir>,
}

impl CostCentersListLedgerRequestSortItem {
    pub fn builder() -> CostCentersListLedgerRequestSortItemBuilder {
        <CostCentersListLedgerRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCentersListLedgerRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<CostCentersListLedgerRequestSortItemDir>,
}

impl CostCentersListLedgerRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: CostCentersListLedgerRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCentersListLedgerRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CostCentersListLedgerRequestSortItemBuilder::field)
    pub fn build(self) -> Result<CostCentersListLedgerRequestSortItem, BuildError> {
        Ok(CostCentersListLedgerRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
