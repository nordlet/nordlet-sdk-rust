pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlIntrastatGenerateDeclarationsResponseRowsItem {
    #[serde(rename = "itemNumber")]
    #[serde(default)]
    pub item_number: i64,
    #[serde(rename = "cnCode")]
    #[serde(default)]
    pub cn_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "originCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<String>,
    #[serde(rename = "partnerVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_vat: Option<String>,
    #[serde(rename = "transactionNature")]
    #[serde(default)]
    pub transaction_nature: String,
    #[serde(rename = "transportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_mode: Option<String>,
    #[serde(rename = "deliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<String>,
    #[serde(rename = "netMassKg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_mass_kg: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistical_value: Option<String>,
}

impl PlIntrastatGenerateDeclarationsResponseRowsItem {
    pub fn builder() -> PlIntrastatGenerateDeclarationsResponseRowsItemBuilder {
        <PlIntrastatGenerateDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlIntrastatGenerateDeclarationsResponseRowsItemBuilder {
    item_number: Option<i64>,
    cn_code: Option<String>,
    description: Option<String>,
    country_code: Option<String>,
    origin_country: Option<String>,
    partner_vat: Option<String>,
    transaction_nature: Option<String>,
    transport_mode: Option<String>,
    delivery_terms: Option<String>,
    net_mass_kg: Option<String>,
    supplementary_unit: Option<String>,
    supplementary_qty: Option<String>,
    invoiced_value: Option<String>,
    statistical_value: Option<String>,
}

impl PlIntrastatGenerateDeclarationsResponseRowsItemBuilder {
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

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
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

    pub fn transaction_nature(mut self, value: impl Into<String>) -> Self {
        self.transaction_nature = Some(value.into());
        self
    }

    pub fn transport_mode(mut self, value: impl Into<String>) -> Self {
        self.transport_mode = Some(value.into());
        self
    }

    pub fn delivery_terms(mut self, value: impl Into<String>) -> Self {
        self.delivery_terms = Some(value.into());
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

    /// Consumes the builder and constructs a [`PlIntrastatGenerateDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_number`](PlIntrastatGenerateDeclarationsResponseRowsItemBuilder::item_number)
    /// - [`cn_code`](PlIntrastatGenerateDeclarationsResponseRowsItemBuilder::cn_code)
    /// - [`country_code`](PlIntrastatGenerateDeclarationsResponseRowsItemBuilder::country_code)
    /// - [`transaction_nature`](PlIntrastatGenerateDeclarationsResponseRowsItemBuilder::transaction_nature)
    /// - [`invoiced_value`](PlIntrastatGenerateDeclarationsResponseRowsItemBuilder::invoiced_value)
    pub fn build(self) -> Result<PlIntrastatGenerateDeclarationsResponseRowsItem, BuildError> {
        Ok(PlIntrastatGenerateDeclarationsResponseRowsItem {
            item_number: self
                .item_number
                .ok_or_else(|| BuildError::missing_field("item_number"))?,
            cn_code: self
                .cn_code
                .ok_or_else(|| BuildError::missing_field("cn_code"))?,
            description: self.description,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            origin_country: self.origin_country,
            partner_vat: self.partner_vat,
            transaction_nature: self
                .transaction_nature
                .ok_or_else(|| BuildError::missing_field("transaction_nature"))?,
            transport_mode: self.transport_mode,
            delivery_terms: self.delivery_terms,
            net_mass_kg: self.net_mass_kg,
            supplementary_unit: self.supplementary_unit,
            supplementary_qty: self.supplementary_qty,
            invoiced_value: self
                .invoiced_value
                .ok_or_else(|| BuildError::missing_field("invoiced_value"))?,
            statistical_value: self.statistical_value,
        })
    }
}
