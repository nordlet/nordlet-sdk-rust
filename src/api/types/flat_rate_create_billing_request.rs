pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlatRateCreateBillingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<FlatRateCreateBillingRequestLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<FlatRateCreateBillingRequestPlan>,
}

impl FlatRateCreateBillingRequest {
    pub fn builder() -> FlatRateCreateBillingRequestBuilder {
        <FlatRateCreateBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlatRateCreateBillingRequestBuilder {
    locale: Option<FlatRateCreateBillingRequestLocale>,
    plan: Option<FlatRateCreateBillingRequestPlan>,
}

impl FlatRateCreateBillingRequestBuilder {
    pub fn locale(mut self, value: FlatRateCreateBillingRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn plan(mut self, value: FlatRateCreateBillingRequestPlan) -> Self {
        self.plan = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FlatRateCreateBillingRequest`].
    pub fn build(self) -> Result<FlatRateCreateBillingRequest, BuildError> {
        Ok(FlatRateCreateBillingRequest {
            locale: self.locale,
            plan: self.plan,
        })
    }
}
