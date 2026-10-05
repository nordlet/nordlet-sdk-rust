pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<SettlementsListBankRequestSortItemDir>,
}

impl SettlementsListBankRequestSortItem {
    pub fn builder() -> SettlementsListBankRequestSortItemBuilder {
        <SettlementsListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<SettlementsListBankRequestSortItemDir>,
}

impl SettlementsListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: SettlementsListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettlementsListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SettlementsListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<SettlementsListBankRequestSortItem, BuildError> {
        Ok(SettlementsListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
