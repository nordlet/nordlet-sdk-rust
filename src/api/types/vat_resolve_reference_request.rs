pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatResolveReferenceRequest {
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "customerCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_country_code: Option<String>,
    #[serde(rename = "customerIsBusiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_is_business: Option<bool>,
    #[serde(rename = "supplyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_type: Option<VatResolveReferenceRequestSupplyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "belowDistanceSalesThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_distance_sales_threshold: Option<bool>,
    #[serde(rename = "facilitatedByMarketplace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facilitated_by_marketplace: Option<bool>,
    #[serde(rename = "actingAsMarketplace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acting_as_marketplace: Option<bool>,
    #[serde(rename = "sellerEstablishedInEu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_established_in_eu: Option<bool>,
    #[serde(rename = "importedConsignmentValueEur")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_consignment_value_eur: Option<String>,
    #[serde(rename = "serviceKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_kind: Option<VatResolveReferenceRequestServiceKind>,
    #[serde(rename = "serviceCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_country_code: Option<String>,
    #[serde(rename = "underlyingSupplierGaveVatNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_supplier_gave_vat_number: Option<bool>,
    #[serde(rename = "underlyingSupplierChargesVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_supplier_charges_vat: Option<bool>,
    #[serde(rename = "goodsKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_kind: Option<VatResolveReferenceRequestGoodsKind>,
    #[serde(rename = "goodsLocationCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_location_country_code: Option<String>,
}

impl VatResolveReferenceRequest {
    pub fn builder() -> VatResolveReferenceRequestBuilder {
        <VatResolveReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatResolveReferenceRequestBuilder {
    partner_id: Option<String>,
    customer_country_code: Option<String>,
    customer_is_business: Option<bool>,
    supply_type: Option<VatResolveReferenceRequestSupplyType>,
    date: Option<NaiveDate>,
    below_distance_sales_threshold: Option<bool>,
    facilitated_by_marketplace: Option<bool>,
    acting_as_marketplace: Option<bool>,
    seller_established_in_eu: Option<bool>,
    imported_consignment_value_eur: Option<String>,
    service_kind: Option<VatResolveReferenceRequestServiceKind>,
    service_country_code: Option<String>,
    underlying_supplier_gave_vat_number: Option<bool>,
    underlying_supplier_charges_vat: Option<bool>,
    goods_kind: Option<VatResolveReferenceRequestGoodsKind>,
    goods_location_country_code: Option<String>,
}

impl VatResolveReferenceRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn customer_country_code(mut self, value: impl Into<String>) -> Self {
        self.customer_country_code = Some(value.into());
        self
    }

    pub fn customer_is_business(mut self, value: bool) -> Self {
        self.customer_is_business = Some(value);
        self
    }

    pub fn supply_type(mut self, value: VatResolveReferenceRequestSupplyType) -> Self {
        self.supply_type = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn below_distance_sales_threshold(mut self, value: bool) -> Self {
        self.below_distance_sales_threshold = Some(value);
        self
    }

    pub fn facilitated_by_marketplace(mut self, value: bool) -> Self {
        self.facilitated_by_marketplace = Some(value);
        self
    }

    pub fn acting_as_marketplace(mut self, value: bool) -> Self {
        self.acting_as_marketplace = Some(value);
        self
    }

    pub fn seller_established_in_eu(mut self, value: bool) -> Self {
        self.seller_established_in_eu = Some(value);
        self
    }

    pub fn imported_consignment_value_eur(mut self, value: impl Into<String>) -> Self {
        self.imported_consignment_value_eur = Some(value.into());
        self
    }

    pub fn service_kind(mut self, value: VatResolveReferenceRequestServiceKind) -> Self {
        self.service_kind = Some(value);
        self
    }

    pub fn service_country_code(mut self, value: impl Into<String>) -> Self {
        self.service_country_code = Some(value.into());
        self
    }

    pub fn underlying_supplier_gave_vat_number(mut self, value: bool) -> Self {
        self.underlying_supplier_gave_vat_number = Some(value);
        self
    }

    pub fn underlying_supplier_charges_vat(mut self, value: bool) -> Self {
        self.underlying_supplier_charges_vat = Some(value);
        self
    }

    pub fn goods_kind(mut self, value: VatResolveReferenceRequestGoodsKind) -> Self {
        self.goods_kind = Some(value);
        self
    }

    pub fn goods_location_country_code(mut self, value: impl Into<String>) -> Self {
        self.goods_location_country_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatResolveReferenceRequest`].
    pub fn build(self) -> Result<VatResolveReferenceRequest, BuildError> {
        Ok(VatResolveReferenceRequest {
            partner_id: self.partner_id,
            customer_country_code: self.customer_country_code,
            customer_is_business: self.customer_is_business,
            supply_type: self.supply_type,
            date: self.date,
            below_distance_sales_threshold: self.below_distance_sales_threshold,
            facilitated_by_marketplace: self.facilitated_by_marketplace,
            acting_as_marketplace: self.acting_as_marketplace,
            seller_established_in_eu: self.seller_established_in_eu,
            imported_consignment_value_eur: self.imported_consignment_value_eur,
            service_kind: self.service_kind,
            service_country_code: self.service_country_code,
            underlying_supplier_gave_vat_number: self.underlying_supplier_gave_vat_number,
            underlying_supplier_charges_vat: self.underlying_supplier_charges_vat,
            goods_kind: self.goods_kind,
            goods_location_country_code: self.goods_location_country_code,
        })
    }
}
