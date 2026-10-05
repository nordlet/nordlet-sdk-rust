pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtCitiesListReferenceResponseRowsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "municipalityCode")]
    #[serde(default)]
    pub municipality_code: String,
}

impl LtCitiesListReferenceResponseRowsItem {
    pub fn builder() -> LtCitiesListReferenceResponseRowsItemBuilder {
        <LtCitiesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtCitiesListReferenceResponseRowsItemBuilder {
    name: Option<String>,
    municipality_code: Option<String>,
}

impl LtCitiesListReferenceResponseRowsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn municipality_code(mut self, value: impl Into<String>) -> Self {
        self.municipality_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtCitiesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](LtCitiesListReferenceResponseRowsItemBuilder::name)
    /// - [`municipality_code`](LtCitiesListReferenceResponseRowsItemBuilder::municipality_code)
    pub fn build(self) -> Result<LtCitiesListReferenceResponseRowsItem, BuildError> {
        Ok(LtCitiesListReferenceResponseRowsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            municipality_code: self
                .municipality_code
                .ok_or_else(|| BuildError::missing_field("municipality_code"))?,
        })
    }
}
