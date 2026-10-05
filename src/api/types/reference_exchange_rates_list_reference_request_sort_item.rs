pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ExchangeRatesListReferenceRequestSortItemDir>,
}

impl ExchangeRatesListReferenceRequestSortItem {
    pub fn builder() -> ExchangeRatesListReferenceRequestSortItemBuilder {
        <ExchangeRatesListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ExchangeRatesListReferenceRequestSortItemDir>,
}

impl ExchangeRatesListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ExchangeRatesListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ExchangeRatesListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ExchangeRatesListReferenceRequestSortItem, BuildError> {
        Ok(ExchangeRatesListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
