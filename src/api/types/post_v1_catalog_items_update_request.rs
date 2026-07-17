pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1CatalogItemsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1CatalogItemsUpdateRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
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
    pub attributes: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, PostV1CatalogItemsUpdateRequestTranslationsValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<PostV1CatalogItemsUpdateRequestComponentsItem>>,
}

impl PostV1CatalogItemsUpdateRequest {
    pub fn builder() -> PostV1CatalogItemsUpdateRequestBuilder {
        <PostV1CatalogItemsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsUpdateRequestBuilder {
    id: Option<String>,
    r#type: Option<PostV1CatalogItemsUpdateRequestType>,
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
    attributes: Option<HashMap<String, String>>,
    translations: Option<HashMap<String, PostV1CatalogItemsUpdateRequestTranslationsValue>>,
    components: Option<Vec<PostV1CatalogItemsUpdateRequestComponentsItem>>,
}

impl PostV1CatalogItemsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1CatalogItemsUpdateRequestType) -> Self {
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

    pub fn attributes(mut self, value: HashMap<String, String>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, PostV1CatalogItemsUpdateRequestTranslationsValue>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn components(mut self, value: Vec<PostV1CatalogItemsUpdateRequestComponentsItem>) -> Self {
        self.components = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsUpdateRequest, BuildError> {
        Ok(PostV1CatalogItemsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type,
            name: self.name,
            code: self.code,
            barcode: self.barcode,
            unit: self.unit,
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
            components: self.components,
        })
    }
}
