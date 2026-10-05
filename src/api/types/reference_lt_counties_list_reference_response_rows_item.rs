pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtCountiesListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "isoCode")]
    #[serde(default)]
    pub iso_code: String,
    #[serde(default)]
    pub name: String,
}

impl LtCountiesListReferenceResponseRowsItem {
    pub fn builder() -> LtCountiesListReferenceResponseRowsItemBuilder {
        <LtCountiesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtCountiesListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    iso_code: Option<String>,
    name: Option<String>,
}

impl LtCountiesListReferenceResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`LtCountiesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LtCountiesListReferenceResponseRowsItemBuilder::code)
    /// - [`iso_code`](LtCountiesListReferenceResponseRowsItemBuilder::iso_code)
    /// - [`name`](LtCountiesListReferenceResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<LtCountiesListReferenceResponseRowsItem, BuildError> {
        Ok(LtCountiesListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            iso_code: self
                .iso_code
                .ok_or_else(|| BuildError::missing_field("iso_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
