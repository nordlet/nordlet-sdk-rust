pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterGroupsListLedgerRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<CostCenterGroupsListLedgerRequestSortItemDir>,
}

impl CostCenterGroupsListLedgerRequestSortItem {
    pub fn builder() -> CostCenterGroupsListLedgerRequestSortItemBuilder {
        <CostCenterGroupsListLedgerRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsListLedgerRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<CostCenterGroupsListLedgerRequestSortItemDir>,
}

impl CostCenterGroupsListLedgerRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: CostCenterGroupsListLedgerRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsListLedgerRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CostCenterGroupsListLedgerRequestSortItemBuilder::field)
    pub fn build(self) -> Result<CostCenterGroupsListLedgerRequestSortItem, BuildError> {
        Ok(CostCenterGroupsListLedgerRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
