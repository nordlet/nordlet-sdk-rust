pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<FeedsConnectionsListBankRequestSortItemDir>,
}

impl FeedsConnectionsListBankRequestSortItem {
    pub fn builder() -> FeedsConnectionsListBankRequestSortItemBuilder {
        <FeedsConnectionsListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<FeedsConnectionsListBankRequestSortItemDir>,
}

impl FeedsConnectionsListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: FeedsConnectionsListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](FeedsConnectionsListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<FeedsConnectionsListBankRequestSortItem, BuildError> {
        Ok(FeedsConnectionsListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
