pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<MandatesListBankRequestSortItemDir>,
}

impl MandatesListBankRequestSortItem {
    pub fn builder() -> MandatesListBankRequestSortItemBuilder {
        <MandatesListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<MandatesListBankRequestSortItemDir>,
}

impl MandatesListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: MandatesListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MandatesListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](MandatesListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<MandatesListBankRequestSortItem, BuildError> {
        Ok(MandatesListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
