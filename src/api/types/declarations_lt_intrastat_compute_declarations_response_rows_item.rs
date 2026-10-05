pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatComputeDeclarationsResponseRowsItem {
    #[serde(rename = "itemNumber")]
    #[serde(default)]
    pub item_number: i64,
    #[serde(rename = "cnCode")]
    #[serde(default)]
    pub cn_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "transactionNature")]
    #[serde(default)]
    pub transaction_nature: String,
    #[serde(rename = "deliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<String>,
    #[serde(rename = "transportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_mode: Option<String>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(default)]
    pub country: String,
    #[serde(rename = "originCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<String>,
    #[serde(rename = "partnerVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_vat: Option<String>,
    #[serde(rename = "netMassKg")]
    #[serde(default)]
    pub net_mass_kg: String,
    #[serde(rename = "supplementaryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_unit: Option<String>,
    #[serde(rename = "supplementaryQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_qty: Option<String>,
    #[serde(rename = "invoicedValue")]
    #[serde(default)]
    pub invoiced_value: String,
    #[serde(rename = "statisticalValue")]
    #[serde(default)]
    pub statistical_value: String,
}

impl LtIntrastatComputeDeclarationsResponseRowsItem {
    pub fn builder() -> LtIntrastatComputeDeclarationsResponseRowsItemBuilder {
        <LtIntrastatComputeDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatComputeDeclarationsResponseRowsItemBuilder {
    item_number: Option<i64>,
    cn_code: Option<String>,
    description: Option<String>,
    transaction_nature: Option<String>,
    delivery_terms: Option<String>,
    transport_mode: Option<String>,
    region_code: Option<String>,
    country: Option<String>,
    origin_country: Option<String>,
    partner_vat: Option<String>,
    net_mass_kg: Option<String>,
    supplementary_unit: Option<String>,
    supplementary_qty: Option<String>,
    invoiced_value: Option<String>,
    statistical_value: Option<String>,
}

impl LtIntrastatComputeDeclarationsResponseRowsItemBuilder {
    pub fn item_number(mut self, value: i64) -> Self {
        self.item_number = Some(value);
        self
    }

    pub fn cn_code(mut self, value: impl Into<String>) -> Self {
        self.cn_code = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn transaction_nature(mut self, value: impl Into<String>) -> Self {
        self.transaction_nature = Some(value.into());
        self
    }

    pub fn delivery_terms(mut self, value: impl Into<String>) -> Self {
        self.delivery_terms = Some(value.into());
        self
    }

    pub fn transport_mode(mut self, value: impl Into<String>) -> Self {
        self.transport_mode = Some(value.into());
        self
    }

    pub fn region_code(mut self, value: impl Into<String>) -> Self {
        self.region_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn origin_country(mut self, value: impl Into<String>) -> Self {
        self.origin_country = Some(value.into());
        self
    }

    pub fn partner_vat(mut self, value: impl Into<String>) -> Self {
        self.partner_vat = Some(value.into());
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

    pub fn supplementary_qty(mut self, value: impl Into<String>) -> Self {
        self.supplementary_qty = Some(value.into());
        self
    }

    pub fn invoiced_value(mut self, value: impl Into<String>) -> Self {
        self.invoiced_value = Some(value.into());
        self
    }

    pub fn statistical_value(mut self, value: impl Into<String>) -> Self {
        self.statistical_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatComputeDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_number`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::item_number)
    /// - [`cn_code`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::cn_code)
    /// - [`transaction_nature`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::transaction_nature)
    /// - [`country`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::country)
    /// - [`net_mass_kg`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::net_mass_kg)
    /// - [`invoiced_value`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::invoiced_value)
    /// - [`statistical_value`](LtIntrastatComputeDeclarationsResponseRowsItemBuilder::statistical_value)
    pub fn build(self) -> Result<LtIntrastatComputeDeclarationsResponseRowsItem, BuildError> {
        Ok(LtIntrastatComputeDeclarationsResponseRowsItem {
            item_number: self
                .item_number
                .ok_or_else(|| BuildError::missing_field("item_number"))?,
            cn_code: self
                .cn_code
                .ok_or_else(|| BuildError::missing_field("cn_code"))?,
            description: self.description,
            transaction_nature: self
                .transaction_nature
                .ok_or_else(|| BuildError::missing_field("transaction_nature"))?,
            delivery_terms: self.delivery_terms,
            transport_mode: self.transport_mode,
            region_code: self.region_code,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            origin_country: self.origin_country,
            partner_vat: self.partner_vat,
            net_mass_kg: self
                .net_mass_kg
                .ok_or_else(|| BuildError::missing_field("net_mass_kg"))?,
            supplementary_unit: self.supplementary_unit,
            supplementary_qty: self.supplementary_qty,
            invoiced_value: self
                .invoiced_value
                .ok_or_else(|| BuildError::missing_field("invoiced_value"))?,
            statistical_value: self
                .statistical_value
                .ok_or_else(|| BuildError::missing_field("statistical_value"))?,
        })
    }
}
