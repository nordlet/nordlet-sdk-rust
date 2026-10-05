pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0564ComputeDeclarationsResponseTotals {
    #[serde(default)]
    pub goods: String,
    #[serde(default)]
    pub triangular: String,
    #[serde(default)]
    pub services: String,
    #[serde(default)]
    pub rows: i64,
}

impl LtFr0564ComputeDeclarationsResponseTotals {
    pub fn builder() -> LtFr0564ComputeDeclarationsResponseTotalsBuilder {
        <LtFr0564ComputeDeclarationsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0564ComputeDeclarationsResponseTotalsBuilder {
    goods: Option<String>,
    triangular: Option<String>,
    services: Option<String>,
    rows: Option<i64>,
}

impl LtFr0564ComputeDeclarationsResponseTotalsBuilder {
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

    /// Consumes the builder and constructs a [`LtFr0564ComputeDeclarationsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`goods`](LtFr0564ComputeDeclarationsResponseTotalsBuilder::goods)
    /// - [`triangular`](LtFr0564ComputeDeclarationsResponseTotalsBuilder::triangular)
    /// - [`services`](LtFr0564ComputeDeclarationsResponseTotalsBuilder::services)
    /// - [`rows`](LtFr0564ComputeDeclarationsResponseTotalsBuilder::rows)
    pub fn build(self) -> Result<LtFr0564ComputeDeclarationsResponseTotals, BuildError> {
        Ok(LtFr0564ComputeDeclarationsResponseTotals {
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
