pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CountriesListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "isEu")]
    #[serde(default)]
    pub is_eu: bool,
    #[serde(rename = "isEea")]
    #[serde(default)]
    pub is_eea: bool,
    #[serde(default)]
    pub names: HashMap<String, String>,
}

impl CountriesListReferenceResponseRowsItem {
    pub fn builder() -> CountriesListReferenceResponseRowsItemBuilder {
        <CountriesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CountriesListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    is_eu: Option<bool>,
    is_eea: Option<bool>,
    names: Option<HashMap<String, String>>,
}

impl CountriesListReferenceResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    pub fn is_eea(mut self, value: bool) -> Self {
        self.is_eea = Some(value);
        self
    }

    pub fn names(mut self, value: HashMap<String, String>) -> Self {
        self.names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CountriesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CountriesListReferenceResponseRowsItemBuilder::code)
    /// - [`is_eu`](CountriesListReferenceResponseRowsItemBuilder::is_eu)
    /// - [`is_eea`](CountriesListReferenceResponseRowsItemBuilder::is_eea)
    /// - [`names`](CountriesListReferenceResponseRowsItemBuilder::names)
    pub fn build(self) -> Result<CountriesListReferenceResponseRowsItem, BuildError> {
        Ok(CountriesListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            is_eu: self
                .is_eu
                .ok_or_else(|| BuildError::missing_field("is_eu"))?,
            is_eea: self
                .is_eea
                .ok_or_else(|| BuildError::missing_field("is_eea"))?,
            names: self
                .names
                .ok_or_else(|| BuildError::missing_field("names"))?,
        })
    }
}
