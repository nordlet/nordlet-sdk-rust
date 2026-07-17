pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1CatalogItemsUpdateResponse {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1CatalogItemsUpdateResponseType,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(default)]
    pub unit: String,
    #[serde(rename = "vatClassifierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_classifier_code: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "salePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price_excl_vat: Option<String>,
    #[serde(rename = "purchasePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price_excl_vat: Option<String>,
    #[serde(rename = "cnCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_code: Option<String>,
    #[serde(rename = "originCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<String>,
    #[serde(rename = "netMassKg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_mass_kg: Option<String>,
    #[serde(rename = "supplementaryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_unit: Option<String>,
    #[serde(rename = "supplementaryQtyPerUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_qty_per_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<PostV1CatalogItemsUpdateResponseTranslationsValue>>>,
    #[serde(default)]
    pub components: Vec<PostV1CatalogItemsUpdateResponseComponentsItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1CatalogItemsUpdateResponse {
    pub fn builder() -> PostV1CatalogItemsUpdateResponseBuilder {
        <PostV1CatalogItemsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsUpdateResponseBuilder {
    id: Option<String>,
    r#type: Option<PostV1CatalogItemsUpdateResponseType>,
    name: Option<String>,
    code: Option<String>,
    barcode: Option<String>,
    unit: Option<String>,
    vat_classifier_code: Option<String>,
    vat_rate_percent: Option<String>,
    sale_price_excl_vat: Option<String>,
    purchase_price_excl_vat: Option<String>,
    cn_code: Option<String>,
    origin_country: Option<String>,
    net_mass_kg: Option<String>,
    supplementary_unit: Option<String>,
    supplementary_qty_per_unit: Option<String>,
    description: Option<String>,
    group_id: Option<String>,
    attributes: Option<HashMap<String, Option<String>>>,
    translations:
        Option<HashMap<String, Option<PostV1CatalogItemsUpdateResponseTranslationsValue>>>,
    components: Option<Vec<PostV1CatalogItemsUpdateResponseComponentsItem>>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1CatalogItemsUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1CatalogItemsUpdateResponseType) -> Self {
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

    pub fn vat_classifier_code(mut self, value: impl Into<String>) -> Self {
        self.vat_classifier_code = Some(value.into());
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

    pub fn cn_code(mut self, value: impl Into<String>) -> Self {
        self.cn_code = Some(value.into());
        self
    }

    pub fn origin_country(mut self, value: impl Into<String>) -> Self {
        self.origin_country = Some(value.into());
        self
    }

    pub fn net_mass_kg(mut self, value: impl Into<String>) -> Self {
        self.net_mass_kg = Some(value.into());
        self
    }

    pub fn supplementary_unit(mut self, value: impl Into<String>) -> Self {
        self.supplementary_unit = Some(value.into());
        self
    }

    pub fn supplementary_qty_per_unit(mut self, value: impl Into<String>) -> Self {
        self.supplementary_qty_per_unit = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, Option<PostV1CatalogItemsUpdateResponseTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn components(
        mut self,
        value: Vec<PostV1CatalogItemsUpdateResponseComponentsItem>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsUpdateResponseBuilder::id)
    /// - [`r#type`](PostV1CatalogItemsUpdateResponseBuilder::r#type)
    /// - [`name`](PostV1CatalogItemsUpdateResponseBuilder::name)
    /// - [`unit`](PostV1CatalogItemsUpdateResponseBuilder::unit)
    /// - [`components`](PostV1CatalogItemsUpdateResponseBuilder::components)
    /// - [`created_at`](PostV1CatalogItemsUpdateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1CatalogItemsUpdateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1CatalogItemsUpdateResponse, BuildError> {
        Ok(PostV1CatalogItemsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            barcode: self.barcode,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            vat_classifier_code: self.vat_classifier_code,
            vat_rate_percent: self.vat_rate_percent,
            sale_price_excl_vat: self.sale_price_excl_vat,
            purchase_price_excl_vat: self.purchase_price_excl_vat,
            cn_code: self.cn_code,
            origin_country: self.origin_country,
            net_mass_kg: self.net_mass_kg,
            supplementary_unit: self.supplementary_unit,
            supplementary_qty_per_unit: self.supplementary_qty_per_unit,
            description: self.description,
            group_id: self.group_id,
            attributes: self.attributes,
            translations: self.translations,
            components: self
                .components
                .ok_or_else(|| BuildError::missing_field("components"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
