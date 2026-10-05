pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrenciesListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<CurrenciesListReferenceRequestSortItemDir>,
}

impl CurrenciesListReferenceRequestSortItem {
    pub fn builder() -> CurrenciesListReferenceRequestSortItemBuilder {
        <CurrenciesListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrenciesListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<CurrenciesListReferenceRequestSortItemDir>,
}

impl CurrenciesListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: CurrenciesListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrenciesListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CurrenciesListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<CurrenciesListReferenceRequestSortItem, BuildError> {
        Ok(CurrenciesListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
