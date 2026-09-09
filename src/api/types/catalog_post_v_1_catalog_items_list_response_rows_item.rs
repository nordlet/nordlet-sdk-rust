pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1CatalogItemsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1CatalogItemsListResponseRowsItemType,
    pub tracking: PostV1CatalogItemsListResponseRowsItemTracking,
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
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<PostV1CatalogItemsListResponseRowsItemTranslationsValue>>>,
    #[serde(default)]
    pub components: Vec<PostV1CatalogItemsListResponseRowsItemComponentsItem>,
    #[serde(rename = "kindId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind_id: Option<String>,
    #[serde(rename = "saleAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_account_code: Option<String>,
    #[serde(rename = "purchaseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_account_code: Option<String>,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "grossMassKg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_mass_kg: Option<String>,
    #[serde(rename = "minQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_quantity: Option<String>,
    #[serde(rename = "costPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<String>,
    #[serde(rename = "isFreePrice")]
    #[serde(default)]
    pub is_free_price: bool,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "isReturnable")]
    #[serde(default)]
    pub is_returnable: bool,
    #[serde(rename = "commentRequired")]
    #[serde(default)]
    pub comment_required: bool,
    #[serde(rename = "priceFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_from: Option<String>,
    #[serde(rename = "priceTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_to: Option<String>,
    #[serde(rename = "minPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<String>,
    #[serde(rename = "discountPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<String>,
    #[serde(rename = "maxDiscountPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount_percent: Option<String>,
    #[serde(rename = "loyaltyPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "ageRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_restriction: Option<i64>,
    #[serde(rename = "packageQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_quantity: Option<String>,
    #[serde(rename = "taraCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tara_code: Option<String>,
    #[serde(rename = "certificateNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    #[serde(rename = "certificateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_date: Option<String>,
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    #[serde(rename = "posFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_flags: Option<HashMap<String, Option<bool>>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1CatalogItemsListResponseRowsItem {
    pub fn builder() -> PostV1CatalogItemsListResponseRowsItemBuilder {
        <PostV1CatalogItemsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsListResponseRowsItemBuilder {
    id: Option<String>,
    r#type: Option<PostV1CatalogItemsListResponseRowsItemType>,
    tracking: Option<PostV1CatalogItemsListResponseRowsItemTracking>,
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
    document_ref: Option<String>,
    translations:
        Option<HashMap<String, Option<PostV1CatalogItemsListResponseRowsItemTranslationsValue>>>,
    components: Option<Vec<PostV1CatalogItemsListResponseRowsItemComponentsItem>>,
    kind_id: Option<String>,
    sale_account_code: Option<String>,
    purchase_account_code: Option<String>,
    expense_account_code: Option<String>,
    manufacturer: Option<String>,
    gross_mass_kg: Option<String>,
    min_quantity: Option<String>,
    cost_price: Option<String>,
    is_free_price: Option<bool>,
    external_id: Option<String>,
    is_returnable: Option<bool>,
    comment_required: Option<bool>,
    price_from: Option<String>,
    price_to: Option<String>,
    min_price: Option<String>,
    discount_percent: Option<String>,
    max_discount_percent: Option<String>,
    loyalty_points: Option<i64>,
    department: Option<String>,
    age_restriction: Option<i64>,
    package_quantity: Option<String>,
    tara_code: Option<String>,
    certificate_number: Option<String>,
    certificate_date: Option<String>,
    valid_from: Option<String>,
    valid_to: Option<String>,
    pos_flags: Option<HashMap<String, Option<bool>>>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1CatalogItemsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1CatalogItemsListResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn tracking(mut self, value: PostV1CatalogItemsListResponseRowsItemTracking) -> Self {
        self.tracking = Some(value);
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

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, Option<PostV1CatalogItemsListResponseRowsItemTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn components(
        mut self,
        value: Vec<PostV1CatalogItemsListResponseRowsItemComponentsItem>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn kind_id(mut self, value: impl Into<String>) -> Self {
        self.kind_id = Some(value.into());
        self
    }

    pub fn sale_account_code(mut self, value: impl Into<String>) -> Self {
        self.sale_account_code = Some(value.into());
        self
    }

    pub fn purchase_account_code(mut self, value: impl Into<String>) -> Self {
        self.purchase_account_code = Some(value.into());
        self
    }

    pub fn expense_account_code(mut self, value: impl Into<String>) -> Self {
        self.expense_account_code = Some(value.into());
        self
    }

    pub fn manufacturer(mut self, value: impl Into<String>) -> Self {
        self.manufacturer = Some(value.into());
        self
    }

    pub fn gross_mass_kg(mut self, value: impl Into<String>) -> Self {
        self.gross_mass_kg = Some(value.into());
        self
    }

    pub fn min_quantity(mut self, value: impl Into<String>) -> Self {
        self.min_quantity = Some(value.into());
        self
    }

    pub fn cost_price(mut self, value: impl Into<String>) -> Self {
        self.cost_price = Some(value.into());
        self
    }

    pub fn is_free_price(mut self, value: bool) -> Self {
        self.is_free_price = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn is_returnable(mut self, value: bool) -> Self {
        self.is_returnable = Some(value);
        self
    }

    pub fn comment_required(mut self, value: bool) -> Self {
        self.comment_required = Some(value);
        self
    }

    pub fn price_from(mut self, value: impl Into<String>) -> Self {
        self.price_from = Some(value.into());
        self
    }

    pub fn price_to(mut self, value: impl Into<String>) -> Self {
        self.price_to = Some(value.into());
        self
    }

    pub fn min_price(mut self, value: impl Into<String>) -> Self {
        self.min_price = Some(value.into());
        self
    }

    pub fn discount_percent(mut self, value: impl Into<String>) -> Self {
        self.discount_percent = Some(value.into());
        self
    }

    pub fn max_discount_percent(mut self, value: impl Into<String>) -> Self {
        self.max_discount_percent = Some(value.into());
        self
    }

    pub fn loyalty_points(mut self, value: i64) -> Self {
        self.loyalty_points = Some(value);
        self
    }

    pub fn department(mut self, value: impl Into<String>) -> Self {
        self.department = Some(value.into());
        self
    }

    pub fn age_restriction(mut self, value: i64) -> Self {
        self.age_restriction = Some(value);
        self
    }

    pub fn package_quantity(mut self, value: impl Into<String>) -> Self {
        self.package_quantity = Some(value.into());
        self
    }

    pub fn tara_code(mut self, value: impl Into<String>) -> Self {
        self.tara_code = Some(value.into());
        self
    }

    pub fn certificate_number(mut self, value: impl Into<String>) -> Self {
        self.certificate_number = Some(value.into());
        self
    }

    pub fn certificate_date(mut self, value: impl Into<String>) -> Self {
        self.certificate_date = Some(value.into());
        self
    }

    pub fn valid_from(mut self, value: impl Into<String>) -> Self {
        self.valid_from = Some(value.into());
        self
    }

    pub fn valid_to(mut self, value: impl Into<String>) -> Self {
        self.valid_to = Some(value.into());
        self
    }

    pub fn pos_flags(mut self, value: HashMap<String, Option<bool>>) -> Self {
        self.pos_flags = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1CatalogItemsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsListResponseRowsItemBuilder::id)
    /// - [`r#type`](PostV1CatalogItemsListResponseRowsItemBuilder::r#type)
    /// - [`tracking`](PostV1CatalogItemsListResponseRowsItemBuilder::tracking)
    /// - [`name`](PostV1CatalogItemsListResponseRowsItemBuilder::name)
    /// - [`unit`](PostV1CatalogItemsListResponseRowsItemBuilder::unit)
    /// - [`components`](PostV1CatalogItemsListResponseRowsItemBuilder::components)
    /// - [`is_free_price`](PostV1CatalogItemsListResponseRowsItemBuilder::is_free_price)
    /// - [`is_returnable`](PostV1CatalogItemsListResponseRowsItemBuilder::is_returnable)
    /// - [`comment_required`](PostV1CatalogItemsListResponseRowsItemBuilder::comment_required)
    /// - [`created_at`](PostV1CatalogItemsListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1CatalogItemsListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1CatalogItemsListResponseRowsItem, BuildError> {
        Ok(PostV1CatalogItemsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            tracking: self
                .tracking
                .ok_or_else(|| BuildError::missing_field("tracking"))?,
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
            document_ref: self.document_ref,
            translations: self.translations,
            components: self
                .components
                .ok_or_else(|| BuildError::missing_field("components"))?,
            kind_id: self.kind_id,
            sale_account_code: self.sale_account_code,
            purchase_account_code: self.purchase_account_code,
            expense_account_code: self.expense_account_code,
            manufacturer: self.manufacturer,
            gross_mass_kg: self.gross_mass_kg,
            min_quantity: self.min_quantity,
            cost_price: self.cost_price,
            is_free_price: self
                .is_free_price
                .ok_or_else(|| BuildError::missing_field("is_free_price"))?,
            external_id: self.external_id,
            is_returnable: self
                .is_returnable
                .ok_or_else(|| BuildError::missing_field("is_returnable"))?,
            comment_required: self
                .comment_required
                .ok_or_else(|| BuildError::missing_field("comment_required"))?,
            price_from: self.price_from,
            price_to: self.price_to,
            min_price: self.min_price,
            discount_percent: self.discount_percent,
            max_discount_percent: self.max_discount_percent,
            loyalty_points: self.loyalty_points,
            department: self.department,
            age_restriction: self.age_restriction,
            package_quantity: self.package_quantity,
            tara_code: self.tara_code,
            certificate_number: self.certificate_number,
            certificate_date: self.certificate_date,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            pos_flags: self.pos_flags,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
