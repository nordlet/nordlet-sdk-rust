pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsOptionsResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub source: PostV1CatalogUnitsOptionsResponseRowsItemSource,
}

impl PostV1CatalogUnitsOptionsResponseRowsItem {
    pub fn builder() -> PostV1CatalogUnitsOptionsResponseRowsItemBuilder {
        <PostV1CatalogUnitsOptionsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsOptionsResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    source: Option<PostV1CatalogUnitsOptionsResponseRowsItemSource>,
}

impl PostV1CatalogUnitsOptionsResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source(mut self, value: PostV1CatalogUnitsOptionsResponseRowsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsOptionsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1CatalogUnitsOptionsResponseRowsItemBuilder::code)
    /// - [`name`](PostV1CatalogUnitsOptionsResponseRowsItemBuilder::name)
    /// - [`source`](PostV1CatalogUnitsOptionsResponseRowsItemBuilder::source)
    pub fn build(self) -> Result<PostV1CatalogUnitsOptionsResponseRowsItem, BuildError> {
        Ok(PostV1CatalogUnitsOptionsResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
