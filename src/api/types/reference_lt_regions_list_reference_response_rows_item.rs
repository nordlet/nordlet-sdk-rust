pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtRegionsListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "isoCode")]
    #[serde(default)]
    pub iso_code: String,
    #[serde(default)]
    pub name: String,
}

impl LtRegionsListReferenceResponseRowsItem {
    pub fn builder() -> LtRegionsListReferenceResponseRowsItemBuilder {
        <LtRegionsListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtRegionsListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    iso_code: Option<String>,
    name: Option<String>,
}

impl LtRegionsListReferenceResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn iso_code(mut self, value: impl Into<String>) -> Self {
        self.iso_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtRegionsListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LtRegionsListReferenceResponseRowsItemBuilder::code)
    /// - [`iso_code`](LtRegionsListReferenceResponseRowsItemBuilder::iso_code)
    /// - [`name`](LtRegionsListReferenceResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<LtRegionsListReferenceResponseRowsItem, BuildError> {
        Ok(LtRegionsListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            iso_code: self
                .iso_code
                .ok_or_else(|| BuildError::missing_field("iso_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
