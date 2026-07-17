pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCurrenciesListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "minorUnits")]
    #[serde(default)]
    pub minor_units: i64,
}

impl PostV1ReferenceCurrenciesListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceCurrenciesListResponseRowsItemBuilder {
        <PostV1ReferenceCurrenciesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCurrenciesListResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    minor_units: Option<i64>,
}

impl PostV1ReferenceCurrenciesListResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn minor_units(mut self, value: i64) -> Self {
        self.minor_units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCurrenciesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceCurrenciesListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1ReferenceCurrenciesListResponseRowsItemBuilder::name)
    /// - [`minor_units`](PostV1ReferenceCurrenciesListResponseRowsItemBuilder::minor_units)
    pub fn build(self) -> Result<PostV1ReferenceCurrenciesListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceCurrenciesListResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            minor_units: self
                .minor_units
                .ok_or_else(|| BuildError::missing_field("minor_units"))?,
        })
    }
}
