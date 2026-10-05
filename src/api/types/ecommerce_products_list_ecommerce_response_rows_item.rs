pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductsListEcommerceResponseRowsItem {
    #[serde(default)]
    pub id: String,
    pub r#type: ProductsListEcommerceResponseRowsItemType,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(default)]
    pub unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<ProductsListEcommerceResponseRowsItemTranslationsValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub components: Vec<ProductsListEcommerceResponseRowsItemComponentsItem>,
    #[serde(rename = "onHand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<String>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ProductsListEcommerceResponseRowsItem {
    pub fn builder() -> ProductsListEcommerceResponseRowsItemBuilder {
        <ProductsListEcommerceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductsListEcommerceResponseRowsItemBuilder {
    id: Option<String>,
    r#type: Option<ProductsListEcommerceResponseRowsItemType>,
    name: Option<String>,
    code: Option<String>,
    barcode: Option<String>,
    unit: Option<String>,
    description: Option<String>,
    translations:
        Option<HashMap<String, Option<ProductsListEcommerceResponseRowsItemTranslationsValue>>>,
    attributes: Option<HashMap<String, Option<String>>>,
    group_id: Option<String>,
    group_name: Option<String>,
    vat_rate_percent: Option<String>,
    price: Option<String>,
    currency: Option<String>,
    components: Option<Vec<ProductsListEcommerceResponseRowsItemComponentsItem>>,
    on_hand: Option<String>,
    reserved: Option<String>,
    available: Option<String>,
    deleted: Option<bool>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ProductsListEcommerceResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ProductsListEcommerceResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn barcode(mut self, value: impl Into<String>) -> Self {
        self.barcode = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, Option<ProductsListEcommerceResponseRowsItemTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn attributes(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn price(mut self, value: impl Into<String>) -> Self {
        self.price = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn components(
        mut self,
        value: Vec<ProductsListEcommerceResponseRowsItemComponentsItem>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn on_hand(mut self, value: impl Into<String>) -> Self {
        self.on_hand = Some(value.into());
        self
    }

    pub fn reserved(mut self, value: impl Into<String>) -> Self {
        self.reserved = Some(value.into());
        self
    }

    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductsListEcommerceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProductsListEcommerceResponseRowsItemBuilder::id)
    /// - [`r#type`](ProductsListEcommerceResponseRowsItemBuilder::r#type)
    /// - [`name`](ProductsListEcommerceResponseRowsItemBuilder::name)
    /// - [`unit`](ProductsListEcommerceResponseRowsItemBuilder::unit)
    /// - [`currency`](ProductsListEcommerceResponseRowsItemBuilder::currency)
    /// - [`components`](ProductsListEcommerceResponseRowsItemBuilder::components)
    /// - [`deleted`](ProductsListEcommerceResponseRowsItemBuilder::deleted)
    /// - [`updated_at`](ProductsListEcommerceResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<ProductsListEcommerceResponseRowsItem, BuildError> {
        Ok(ProductsListEcommerceResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            barcode: self.barcode,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            description: self.description,
            translations: self.translations,
            attributes: self.attributes,
            group_id: self.group_id,
            group_name: self.group_name,
            vat_rate_percent: self.vat_rate_percent,
            price: self.price,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            components: self
                .components
                .ok_or_else(|| BuildError::missing_field("components"))?,
            on_hand: self.on_hand,
            reserved: self.reserved,
            available: self.available,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
