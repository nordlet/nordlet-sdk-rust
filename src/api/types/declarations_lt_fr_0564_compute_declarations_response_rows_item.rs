pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0564ComputeDeclarationsResponseRowsItem {
    #[serde(rename = "vatCode")]
    #[serde(default)]
    pub vat_code: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub goods: String,
    #[serde(default)]
    pub triangular: String,
    #[serde(default)]
    pub services: String,
}

impl LtFr0564ComputeDeclarationsResponseRowsItem {
    pub fn builder() -> LtFr0564ComputeDeclarationsResponseRowsItemBuilder {
        <LtFr0564ComputeDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0564ComputeDeclarationsResponseRowsItemBuilder {
    vat_code: Option<String>,
    partner_name: Option<String>,
    country_code: Option<String>,
    goods: Option<String>,
    triangular: Option<String>,
    services: Option<String>,
}

impl LtFr0564ComputeDeclarationsResponseRowsItemBuilder {
    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn goods(mut self, value: impl Into<String>) -> Self {
        self.goods = Some(value.into());
        self
    }

    pub fn triangular(mut self, value: impl Into<String>) -> Self {
        self.triangular = Some(value.into());
        self
    }

    pub fn services(mut self, value: impl Into<String>) -> Self {
        self.services = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtFr0564ComputeDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_code`](LtFr0564ComputeDeclarationsResponseRowsItemBuilder::vat_code)
    /// - [`partner_name`](LtFr0564ComputeDeclarationsResponseRowsItemBuilder::partner_name)
    /// - [`country_code`](LtFr0564ComputeDeclarationsResponseRowsItemBuilder::country_code)
    /// - [`goods`](LtFr0564ComputeDeclarationsResponseRowsItemBuilder::goods)
    /// - [`triangular`](LtFr0564ComputeDeclarationsResponseRowsItemBuilder::triangular)
    /// - [`services`](LtFr0564ComputeDeclarationsResponseRowsItemBuilder::services)
    pub fn build(self) -> Result<LtFr0564ComputeDeclarationsResponseRowsItem, BuildError> {
        Ok(LtFr0564ComputeDeclarationsResponseRowsItem {
            vat_code: self
                .vat_code
                .ok_or_else(|| BuildError::missing_field("vat_code"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            goods: self
                .goods
                .ok_or_else(|| BuildError::missing_field("goods"))?,
            triangular: self
                .triangular
                .ok_or_else(|| BuildError::missing_field("triangular"))?,
            services: self
                .services
                .ok_or_else(|| BuildError::missing_field("services"))?,
        })
    }
}
