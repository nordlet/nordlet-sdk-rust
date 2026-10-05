pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtIntrastatComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub flow: LtIntrastatComputeDeclarationsRequestFlow,
    #[serde(rename = "transactionNature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_nature: Option<String>,
    #[serde(rename = "deliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<String>,
    #[serde(rename = "transportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_mode: Option<LtIntrastatComputeDeclarationsRequestTransportMode>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(rename = "statisticalValueRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistical_value_required: Option<bool>,
    #[serde(rename = "preparationTimeHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparation_time_hours: Option<i64>,
    #[serde(rename = "preparationTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparation_time_minutes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl LtIntrastatComputeDeclarationsRequest {
    pub fn builder() -> LtIntrastatComputeDeclarationsRequestBuilder {
        <LtIntrastatComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    flow: Option<LtIntrastatComputeDeclarationsRequestFlow>,
    transaction_nature: Option<String>,
    delivery_terms: Option<String>,
    transport_mode: Option<LtIntrastatComputeDeclarationsRequestTransportMode>,
    region_code: Option<String>,
    statistical_value_required: Option<bool>,
    preparation_time_hours: Option<i64>,
    preparation_time_minutes: Option<i64>,
    persist: Option<bool>,
}

impl LtIntrastatComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn flow(mut self, value: LtIntrastatComputeDeclarationsRequestFlow) -> Self {
        self.flow = Some(value);
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

    pub fn transport_mode(
        mut self,
        value: LtIntrastatComputeDeclarationsRequestTransportMode,
    ) -> Self {
        self.transport_mode = Some(value);
        self
    }

    pub fn region_code(mut self, value: impl Into<String>) -> Self {
        self.region_code = Some(value.into());
        self
    }

    pub fn statistical_value_required(mut self, value: bool) -> Self {
        self.statistical_value_required = Some(value);
        self
    }

    pub fn preparation_time_hours(mut self, value: i64) -> Self {
        self.preparation_time_hours = Some(value);
        self
    }

    pub fn preparation_time_minutes(mut self, value: i64) -> Self {
        self.preparation_time_minutes = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtIntrastatComputeDeclarationsRequestBuilder::year)
    /// - [`month`](LtIntrastatComputeDeclarationsRequestBuilder::month)
    /// - [`flow`](LtIntrastatComputeDeclarationsRequestBuilder::flow)
    pub fn build(self) -> Result<LtIntrastatComputeDeclarationsRequest, BuildError> {
        Ok(LtIntrastatComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            flow: self.flow.ok_or_else(|| BuildError::missing_field("flow"))?,
            transaction_nature: self.transaction_nature,
            delivery_terms: self.delivery_terms,
            transport_mode: self.transport_mode,
            region_code: self.region_code,
            statistical_value_required: self.statistical_value_required,
            preparation_time_hours: self.preparation_time_hours,
            preparation_time_minutes: self.preparation_time_minutes,
            persist: self.persist,
        })
    }
}
