pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsSuppliersListCatalogRequest {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
}

impl ItemsSuppliersListCatalogRequest {
    pub fn builder() -> ItemsSuppliersListCatalogRequestBuilder {
        <ItemsSuppliersListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsSuppliersListCatalogRequestBuilder {
    item_id: Option<String>,
    partner_id: Option<String>,
}

impl ItemsSuppliersListCatalogRequestBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsSuppliersListCatalogRequest`].
    pub fn build(self) -> Result<ItemsSuppliersListCatalogRequest, BuildError> {
        Ok(ItemsSuppliersListCatalogRequest {
            item_id: self.item_id,
            partner_id: self.partner_id,
        })
    }
}
