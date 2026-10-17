pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SecuritiesOperationsListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<SecuritiesOperationsListBankRequestSortItemDir>,
}

impl SecuritiesOperationsListBankRequestSortItem {
    pub fn builder() -> SecuritiesOperationsListBankRequestSortItemBuilder {
        <SecuritiesOperationsListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesOperationsListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<SecuritiesOperationsListBankRequestSortItemDir>,
}

impl SecuritiesOperationsListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: SecuritiesOperationsListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesOperationsListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SecuritiesOperationsListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<SecuritiesOperationsListBankRequestSortItem, BuildError> {
        Ok(SecuritiesOperationsListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
