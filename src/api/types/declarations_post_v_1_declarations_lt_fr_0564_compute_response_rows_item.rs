pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0564ComputeResponseRowsItem {
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

impl PostV1DeclarationsLtFr0564ComputeResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder {
        <PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder {
    vat_code: Option<String>,
    partner_name: Option<String>,
    country_code: Option<String>,
    goods: Option<String>,
    triangular: Option<String>,
    services: Option<String>,
}

impl PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0564ComputeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_code`](PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder::vat_code)
    /// - [`partner_name`](PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder::partner_name)
    /// - [`country_code`](PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder::country_code)
    /// - [`goods`](PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder::goods)
    /// - [`triangular`](PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder::triangular)
    /// - [`services`](PostV1DeclarationsLtFr0564ComputeResponseRowsItemBuilder::services)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0564ComputeResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsLtFr0564ComputeResponseRowsItem {
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
