pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemGroupsCreateCatalogRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl ItemGroupsCreateCatalogRequest {
    pub fn builder() -> ItemGroupsCreateCatalogRequestBuilder {
        <ItemGroupsCreateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemGroupsCreateCatalogRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    parent_id: Option<String>,
}

impl ItemGroupsCreateCatalogRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemGroupsCreateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ItemGroupsCreateCatalogRequestBuilder::code)
    /// - [`name`](ItemGroupsCreateCatalogRequestBuilder::name)
    pub fn build(self) -> Result<ItemGroupsCreateCatalogRequest, BuildError> {
        Ok(ItemGroupsCreateCatalogRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_id: self.parent_id,
        })
    }
}
