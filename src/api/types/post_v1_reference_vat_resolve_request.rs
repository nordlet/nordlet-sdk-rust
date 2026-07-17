pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatResolveRequest {
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
    pub supply_type: Option<PostV1ReferenceVatResolveRequestSupplyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
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
}

impl PostV1ReferenceVatResolveRequest {
    pub fn builder() -> PostV1ReferenceVatResolveRequestBuilder {
        <PostV1ReferenceVatResolveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatResolveRequestBuilder {
    partner_id: Option<String>,
    customer_country_code: Option<String>,
    customer_is_business: Option<bool>,
    supply_type: Option<PostV1ReferenceVatResolveRequestSupplyType>,
    date: Option<String>,
    below_distance_sales_threshold: Option<bool>,
    facilitated_by_marketplace: Option<bool>,
    acting_as_marketplace: Option<bool>,
    seller_established_in_eu: Option<bool>,
    imported_consignment_value_eur: Option<String>,
}

impl PostV1ReferenceVatResolveRequestBuilder {
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

    pub fn supply_type(mut self, value: PostV1ReferenceVatResolveRequestSupplyType) -> Self {
        self.supply_type = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ReferenceVatResolveRequest`].
    pub fn build(self) -> Result<PostV1ReferenceVatResolveRequest, BuildError> {
        Ok(PostV1ReferenceVatResolveRequest {
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
        })
    }
}
