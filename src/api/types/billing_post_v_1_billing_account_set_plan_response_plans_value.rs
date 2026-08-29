pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1BillingAccountSetPlanResponsePlansValue {
    #[serde(rename = "monthlyFeeEur")]
    #[serde(default)]
    pub monthly_fee_eur: String,
    #[serde(rename = "includedRequests")]
    #[serde(default)]
    pub included_requests: i64,
    #[serde(rename = "requestOverageEur")]
    #[serde(default)]
    pub request_overage_eur: String,
    #[serde(rename = "includedDatabaseBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub included_database_bytes: f64,
    #[serde(rename = "includedFileBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub included_file_bytes: f64,
}

impl PostV1BillingAccountSetPlanResponsePlansValue {
    pub fn builder() -> PostV1BillingAccountSetPlanResponsePlansValueBuilder {
        <PostV1BillingAccountSetPlanResponsePlansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountSetPlanResponsePlansValueBuilder {
    monthly_fee_eur: Option<String>,
    included_requests: Option<i64>,
    request_overage_eur: Option<String>,
    included_database_bytes: Option<f64>,
    included_file_bytes: Option<f64>,
}

impl PostV1BillingAccountSetPlanResponsePlansValueBuilder {
    pub fn monthly_fee_eur(mut self, value: impl Into<String>) -> Self {
        self.monthly_fee_eur = Some(value.into());
        self
    }

    pub fn included_requests(mut self, value: i64) -> Self {
        self.included_requests = Some(value);
        self
    }

    pub fn request_overage_eur(mut self, value: impl Into<String>) -> Self {
        self.request_overage_eur = Some(value.into());
        self
    }

    pub fn included_database_bytes(mut self, value: f64) -> Self {
        self.included_database_bytes = Some(value);
        self
    }

    pub fn included_file_bytes(mut self, value: f64) -> Self {
        self.included_file_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingAccountSetPlanResponsePlansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`monthly_fee_eur`](PostV1BillingAccountSetPlanResponsePlansValueBuilder::monthly_fee_eur)
    /// - [`included_requests`](PostV1BillingAccountSetPlanResponsePlansValueBuilder::included_requests)
    /// - [`request_overage_eur`](PostV1BillingAccountSetPlanResponsePlansValueBuilder::request_overage_eur)
    /// - [`included_database_bytes`](PostV1BillingAccountSetPlanResponsePlansValueBuilder::included_database_bytes)
    /// - [`included_file_bytes`](PostV1BillingAccountSetPlanResponsePlansValueBuilder::included_file_bytes)
    pub fn build(self) -> Result<PostV1BillingAccountSetPlanResponsePlansValue, BuildError> {
        Ok(PostV1BillingAccountSetPlanResponsePlansValue {
            monthly_fee_eur: self
                .monthly_fee_eur
                .ok_or_else(|| BuildError::missing_field("monthly_fee_eur"))?,
            included_requests: self
                .included_requests
                .ok_or_else(|| BuildError::missing_field("included_requests"))?,
            request_overage_eur: self
                .request_overage_eur
                .ok_or_else(|| BuildError::missing_field("request_overage_eur"))?,
            included_database_bytes: self
                .included_database_bytes
                .ok_or_else(|| BuildError::missing_field("included_database_bytes"))?,
            included_file_bytes: self
                .included_file_bytes
                .ok_or_else(|| BuildError::missing_field("included_file_bytes"))?,
        })
    }
}
