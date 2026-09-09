pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtCountiesListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "isoCode")]
    #[serde(default)]
    pub iso_code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1ReferenceLtCountiesListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceLtCountiesListResponseRowsItemBuilder {
        <PostV1ReferenceLtCountiesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtCountiesListResponseRowsItemBuilder {
    code: Option<String>,
    iso_code: Option<String>,
    name: Option<String>,
}

impl PostV1ReferenceLtCountiesListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceLtCountiesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceLtCountiesListResponseRowsItemBuilder::code)
    /// - [`iso_code`](PostV1ReferenceLtCountiesListResponseRowsItemBuilder::iso_code)
    /// - [`name`](PostV1ReferenceLtCountiesListResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1ReferenceLtCountiesListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceLtCountiesListResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            iso_code: self
                .iso_code
                .ok_or_else(|| BuildError::missing_field("iso_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
