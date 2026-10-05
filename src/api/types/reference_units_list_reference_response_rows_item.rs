pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "nameLt")]
    #[serde(default)]
    pub name_lt: String,
    #[serde(rename = "nameEn")]
    #[serde(default)]
    pub name_en: String,
}

impl UnitsListReferenceResponseRowsItem {
    pub fn builder() -> UnitsListReferenceResponseRowsItemBuilder {
        <UnitsListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    name_lt: Option<String>,
    name_en: Option<String>,
}

impl UnitsListReferenceResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name_lt(mut self, value: impl Into<String>) -> Self {
        self.name_lt = Some(value.into());
        self
    }

    pub fn name_en(mut self, value: impl Into<String>) -> Self {
        self.name_en = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitsListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UnitsListReferenceResponseRowsItemBuilder::code)
    /// - [`name_lt`](UnitsListReferenceResponseRowsItemBuilder::name_lt)
    /// - [`name_en`](UnitsListReferenceResponseRowsItemBuilder::name_en)
    pub fn build(self) -> Result<UnitsListReferenceResponseRowsItem, BuildError> {
        Ok(UnitsListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name_lt: self
                .name_lt
                .ok_or_else(|| BuildError::missing_field("name_lt"))?,
            name_en: self
                .name_en
                .ok_or_else(|| BuildError::missing_field("name_en"))?,
        })
    }
}
