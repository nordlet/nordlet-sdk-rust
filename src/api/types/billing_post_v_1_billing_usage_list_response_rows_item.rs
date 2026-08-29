pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BillingUsageListResponseRowsItem {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(default)]
    pub date: String,
    pub metric: PostV1BillingUsageListResponseRowsItemMetric,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
}

impl PostV1BillingUsageListResponseRowsItem {
    pub fn builder() -> PostV1BillingUsageListResponseRowsItemBuilder {
        <PostV1BillingUsageListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingUsageListResponseRowsItemBuilder {
    company_id: Option<String>,
    date: Option<String>,
    metric: Option<PostV1BillingUsageListResponseRowsItemMetric>,
    quantity: Option<f64>,
}

impl PostV1BillingUsageListResponseRowsItemBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn metric(mut self, value: PostV1BillingUsageListResponseRowsItemMetric) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingUsageListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](PostV1BillingUsageListResponseRowsItemBuilder::company_id)
    /// - [`date`](PostV1BillingUsageListResponseRowsItemBuilder::date)
    /// - [`metric`](PostV1BillingUsageListResponseRowsItemBuilder::metric)
    /// - [`quantity`](PostV1BillingUsageListResponseRowsItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1BillingUsageListResponseRowsItem, BuildError> {
        Ok(PostV1BillingUsageListResponseRowsItem {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            metric: self
                .metric
                .ok_or_else(|| BuildError::missing_field("metric"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
