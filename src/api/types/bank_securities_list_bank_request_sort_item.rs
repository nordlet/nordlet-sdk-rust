pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SecuritiesListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<SecuritiesListBankRequestSortItemDir>,
}

impl SecuritiesListBankRequestSortItem {
    pub fn builder() -> SecuritiesListBankRequestSortItemBuilder {
        <SecuritiesListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<SecuritiesListBankRequestSortItemDir>,
}

impl SecuritiesListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: SecuritiesListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SecuritiesListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<SecuritiesListBankRequestSortItem, BuildError> {
        Ok(SecuritiesListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
