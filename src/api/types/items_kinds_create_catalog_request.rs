pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsKindsCreateCatalogRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "saftType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saft_type: Option<ItemsKindsCreateCatalogRequestSaftType>,
    #[serde(rename = "quantityAccounting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_accounting: Option<bool>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl ItemsKindsCreateCatalogRequest {
    pub fn builder() -> ItemsKindsCreateCatalogRequestBuilder {
        <ItemsKindsCreateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsCreateCatalogRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    saft_type: Option<ItemsKindsCreateCatalogRequestSaftType>,
    quantity_accounting: Option<bool>,
    sort_order: Option<i64>,
}

impl ItemsKindsCreateCatalogRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn saft_type(mut self, value: ItemsKindsCreateCatalogRequestSaftType) -> Self {
        self.saft_type = Some(value);
        self
    }

    pub fn quantity_accounting(mut self, value: bool) -> Self {
        self.quantity_accounting = Some(value);
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsKindsCreateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ItemsKindsCreateCatalogRequestBuilder::code)
    /// - [`name`](ItemsKindsCreateCatalogRequestBuilder::name)
    pub fn build(self) -> Result<ItemsKindsCreateCatalogRequest, BuildError> {
        Ok(ItemsKindsCreateCatalogRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            saft_type: self.saft_type,
            quantity_accounting: self.quantity_accounting,
            sort_order: self.sort_order,
        })
    }
}
