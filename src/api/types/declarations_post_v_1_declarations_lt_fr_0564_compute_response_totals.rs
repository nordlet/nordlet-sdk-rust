pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0564ComputeResponseTotals {
    #[serde(default)]
    pub goods: String,
    #[serde(default)]
    pub triangular: String,
    #[serde(default)]
    pub services: String,
    #[serde(default)]
    pub rows: i64,
}

impl PostV1DeclarationsLtFr0564ComputeResponseTotals {
    pub fn builder() -> PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder {
        <PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder {
    goods: Option<String>,
    triangular: Option<String>,
    services: Option<String>,
    rows: Option<i64>,
}

impl PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder {
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

    pub fn rows(mut self, value: i64) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0564ComputeResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`goods`](PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder::goods)
    /// - [`triangular`](PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder::triangular)
    /// - [`services`](PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder::services)
    /// - [`rows`](PostV1DeclarationsLtFr0564ComputeResponseTotalsBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0564ComputeResponseTotals, BuildError> {
        Ok(PostV1DeclarationsLtFr0564ComputeResponseTotals {
            goods: self
                .goods
                .ok_or_else(|| BuildError::missing_field("goods"))?,
            triangular: self
                .triangular
                .ok_or_else(|| BuildError::missing_field("triangular"))?,
            services: self
                .services
                .ok_or_else(|| BuildError::missing_field("services"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
