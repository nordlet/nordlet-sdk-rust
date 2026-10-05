pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountsListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<BankAccountsListPartnersRequestSortItemDir>,
}

impl BankAccountsListPartnersRequestSortItem {
    pub fn builder() -> BankAccountsListPartnersRequestSortItemBuilder {
        <BankAccountsListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<BankAccountsListPartnersRequestSortItemDir>,
}

impl BankAccountsListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: BankAccountsListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BankAccountsListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<BankAccountsListPartnersRequestSortItem, BuildError> {
        Ok(BankAccountsListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
