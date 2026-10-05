pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsageListBillingResponseRowsItem {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    pub metric: UsageListBillingResponseRowsItemMetric,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
}

impl UsageListBillingResponseRowsItem {
    pub fn builder() -> UsageListBillingResponseRowsItemBuilder {
        <UsageListBillingResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageListBillingResponseRowsItemBuilder {
    company_id: Option<String>,
    date: Option<NaiveDate>,
    metric: Option<UsageListBillingResponseRowsItemMetric>,
    quantity: Option<f64>,
}

impl UsageListBillingResponseRowsItemBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn metric(mut self, value: UsageListBillingResponseRowsItemMetric) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageListBillingResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](UsageListBillingResponseRowsItemBuilder::company_id)
    /// - [`date`](UsageListBillingResponseRowsItemBuilder::date)
    /// - [`metric`](UsageListBillingResponseRowsItemBuilder::metric)
    /// - [`quantity`](UsageListBillingResponseRowsItemBuilder::quantity)
    pub fn build(self) -> Result<UsageListBillingResponseRowsItem, BuildError> {
        Ok(UsageListBillingResponseRowsItem {
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
