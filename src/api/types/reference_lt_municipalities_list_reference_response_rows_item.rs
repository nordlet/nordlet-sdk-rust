pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtMunicipalitiesListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "countyCode")]
    #[serde(default)]
    pub county_code: String,
}

impl LtMunicipalitiesListReferenceResponseRowsItem {
    pub fn builder() -> LtMunicipalitiesListReferenceResponseRowsItemBuilder {
        <LtMunicipalitiesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtMunicipalitiesListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    county_code: Option<String>,
}

impl LtMunicipalitiesListReferenceResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn county_code(mut self, value: impl Into<String>) -> Self {
        self.county_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtMunicipalitiesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LtMunicipalitiesListReferenceResponseRowsItemBuilder::code)
    /// - [`name`](LtMunicipalitiesListReferenceResponseRowsItemBuilder::name)
    /// - [`county_code`](LtMunicipalitiesListReferenceResponseRowsItemBuilder::county_code)
    pub fn build(self) -> Result<LtMunicipalitiesListReferenceResponseRowsItem, BuildError> {
        Ok(LtMunicipalitiesListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            county_code: self
                .county_code
                .ok_or_else(|| BuildError::missing_field("county_code"))?,
        })
    }
}
