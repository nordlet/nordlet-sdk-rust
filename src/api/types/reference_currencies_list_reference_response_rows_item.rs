pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrenciesListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "minorUnits")]
    #[serde(default)]
    pub minor_units: i64,
}

impl CurrenciesListReferenceResponseRowsItem {
    pub fn builder() -> CurrenciesListReferenceResponseRowsItemBuilder {
        <CurrenciesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrenciesListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    minor_units: Option<i64>,
}

impl CurrenciesListReferenceResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn minor_units(mut self, value: i64) -> Self {
        self.minor_units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrenciesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CurrenciesListReferenceResponseRowsItemBuilder::code)
    /// - [`name`](CurrenciesListReferenceResponseRowsItemBuilder::name)
    /// - [`minor_units`](CurrenciesListReferenceResponseRowsItemBuilder::minor_units)
    pub fn build(self) -> Result<CurrenciesListReferenceResponseRowsItem, BuildError> {
        Ok(CurrenciesListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            minor_units: self
                .minor_units
                .ok_or_else(|| BuildError::missing_field("minor_units"))?,
        })
    }
}
