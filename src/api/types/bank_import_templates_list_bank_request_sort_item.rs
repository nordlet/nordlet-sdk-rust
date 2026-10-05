pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ImportTemplatesListBankRequestSortItemDir>,
}

impl ImportTemplatesListBankRequestSortItem {
    pub fn builder() -> ImportTemplatesListBankRequestSortItemBuilder {
        <ImportTemplatesListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ImportTemplatesListBankRequestSortItemDir>,
}

impl ImportTemplatesListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ImportTemplatesListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ImportTemplatesListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ImportTemplatesListBankRequestSortItem, BuildError> {
        Ok(ImportTemplatesListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
