pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestItemsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1MigrationBooksImportRequestItemsItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "salePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price_excl_vat: Option<String>,
    #[serde(rename = "purchasePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price_excl_vat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1MigrationBooksImportRequestItemsItem {
    pub fn builder() -> PostV1MigrationBooksImportRequestItemsItemBuilder {
        <PostV1MigrationBooksImportRequestItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestItemsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1MigrationBooksImportRequestItemsItemType>,
    unit: Option<String>,
    barcode: Option<String>,
    vat_rate_percent: Option<String>,
    sale_price_excl_vat: Option<String>,
    purchase_price_excl_vat: Option<String>,
    description: Option<String>,
}

impl PostV1MigrationBooksImportRequestItemsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1MigrationBooksImportRequestItemsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn barcode(mut self, value: impl Into<String>) -> Self {
        self.barcode = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn sale_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.sale_price_excl_vat = Some(value.into());
        self
    }

    pub fn purchase_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.purchase_price_excl_vat = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1MigrationBooksImportRequestItemsItemBuilder::code)
    /// - [`name`](PostV1MigrationBooksImportRequestItemsItemBuilder::name)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestItemsItem, BuildError> {
        Ok(PostV1MigrationBooksImportRequestItemsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
            unit: self.unit,
            barcode: self.barcode,
            vat_rate_percent: self.vat_rate_percent,
            sale_price_excl_vat: self.sale_price_excl_vat,
            purchase_price_excl_vat: self.purchase_price_excl_vat,
            description: self.description,
        })
    }
}
