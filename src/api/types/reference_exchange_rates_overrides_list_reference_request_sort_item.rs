pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesOverridesListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ExchangeRatesOverridesListReferenceRequestSortItemDir>,
}

impl ExchangeRatesOverridesListReferenceRequestSortItem {
    pub fn builder() -> ExchangeRatesOverridesListReferenceRequestSortItemBuilder {
        <ExchangeRatesOverridesListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesOverridesListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ExchangeRatesOverridesListReferenceRequestSortItemDir>,
}

impl ExchangeRatesOverridesListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ExchangeRatesOverridesListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesOverridesListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ExchangeRatesOverridesListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ExchangeRatesOverridesListReferenceRequestSortItem, BuildError> {
        Ok(ExchangeRatesOverridesListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
