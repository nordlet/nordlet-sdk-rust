pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnersListLedgerRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<OwnersListLedgerRequestSortItemDir>,
}

impl OwnersListLedgerRequestSortItem {
    pub fn builder() -> OwnersListLedgerRequestSortItemBuilder {
        <OwnersListLedgerRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersListLedgerRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<OwnersListLedgerRequestSortItemDir>,
}

impl OwnersListLedgerRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: OwnersListLedgerRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnersListLedgerRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OwnersListLedgerRequestSortItemBuilder::field)
    pub fn build(self) -> Result<OwnersListLedgerRequestSortItem, BuildError> {
        Ok(OwnersListLedgerRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
