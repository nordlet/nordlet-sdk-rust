pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtCitiesListResponseRowsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "municipalityCode")]
    #[serde(default)]
    pub municipality_code: String,
}

impl PostV1ReferenceLtCitiesListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceLtCitiesListResponseRowsItemBuilder {
        <PostV1ReferenceLtCitiesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtCitiesListResponseRowsItemBuilder {
    name: Option<String>,
    municipality_code: Option<String>,
}

impl PostV1ReferenceLtCitiesListResponseRowsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn municipality_code(mut self, value: impl Into<String>) -> Self {
        self.municipality_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtCitiesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1ReferenceLtCitiesListResponseRowsItemBuilder::name)
    /// - [`municipality_code`](PostV1ReferenceLtCitiesListResponseRowsItemBuilder::municipality_code)
    pub fn build(self) -> Result<PostV1ReferenceLtCitiesListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceLtCitiesListResponseRowsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            municipality_code: self
                .municipality_code
                .ok_or_else(|| BuildError::missing_field("municipality_code"))?,
        })
    }
}
