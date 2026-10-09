pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpenseReportsListCashRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ExpenseReportsListCashRequestSortItemDir>,
}

impl ExpenseReportsListCashRequestSortItem {
    pub fn builder() -> ExpenseReportsListCashRequestSortItemBuilder {
        <ExpenseReportsListCashRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsListCashRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ExpenseReportsListCashRequestSortItemDir>,
}

impl ExpenseReportsListCashRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ExpenseReportsListCashRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsListCashRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ExpenseReportsListCashRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ExpenseReportsListCashRequestSortItem, BuildError> {
        Ok(ExpenseReportsListCashRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
