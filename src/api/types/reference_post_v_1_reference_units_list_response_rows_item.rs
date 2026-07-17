pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceUnitsListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "nameLt")]
    #[serde(default)]
    pub name_lt: String,
    #[serde(rename = "nameEn")]
    #[serde(default)]
    pub name_en: String,
}

impl PostV1ReferenceUnitsListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceUnitsListResponseRowsItemBuilder {
        <PostV1ReferenceUnitsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceUnitsListResponseRowsItemBuilder {
    code: Option<String>,
    name_lt: Option<String>,
    name_en: Option<String>,
}

impl PostV1ReferenceUnitsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceUnitsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceUnitsListResponseRowsItemBuilder::code)
    /// - [`name_lt`](PostV1ReferenceUnitsListResponseRowsItemBuilder::name_lt)
    /// - [`name_en`](PostV1ReferenceUnitsListResponseRowsItemBuilder::name_en)
    pub fn build(self) -> Result<PostV1ReferenceUnitsListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceUnitsListResponseRowsItem {
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
