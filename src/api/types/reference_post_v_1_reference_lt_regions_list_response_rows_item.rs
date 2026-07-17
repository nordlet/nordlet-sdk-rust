pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtRegionsListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "isoCode")]
    #[serde(default)]
    pub iso_code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1ReferenceLtRegionsListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceLtRegionsListResponseRowsItemBuilder {
        <PostV1ReferenceLtRegionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtRegionsListResponseRowsItemBuilder {
    code: Option<String>,
    iso_code: Option<String>,
    name: Option<String>,
}

impl PostV1ReferenceLtRegionsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceLtRegionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceLtRegionsListResponseRowsItemBuilder::code)
    /// - [`iso_code`](PostV1ReferenceLtRegionsListResponseRowsItemBuilder::iso_code)
    /// - [`name`](PostV1ReferenceLtRegionsListResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1ReferenceLtRegionsListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceLtRegionsListResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            iso_code: self
                .iso_code
                .ok_or_else(|| BuildError::missing_field("iso_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
