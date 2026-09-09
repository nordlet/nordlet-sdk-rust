pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtMunicipalitiesListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "countyCode")]
    #[serde(default)]
    pub county_code: String,
}

impl PostV1ReferenceLtMunicipalitiesListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder {
        <PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    county_code: Option<String>,
}

impl PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceLtMunicipalitiesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder::name)
    /// - [`county_code`](PostV1ReferenceLtMunicipalitiesListResponseRowsItemBuilder::county_code)
    pub fn build(self) -> Result<PostV1ReferenceLtMunicipalitiesListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceLtMunicipalitiesListResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            county_code: self
                .county_code
                .ok_or_else(|| BuildError::missing_field("county_code"))?,
        })
    }
}
