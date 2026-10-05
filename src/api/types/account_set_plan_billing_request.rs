pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountSetPlanBillingRequest {
    pub plan: AccountSetPlanBillingRequestPlan,
}

impl AccountSetPlanBillingRequest {
    pub fn builder() -> AccountSetPlanBillingRequestBuilder {
        <AccountSetPlanBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetPlanBillingRequestBuilder {
    plan: Option<AccountSetPlanBillingRequestPlan>,
}

impl AccountSetPlanBillingRequestBuilder {
    pub fn plan(mut self, value: AccountSetPlanBillingRequestPlan) -> Self {
        self.plan = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSetPlanBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](AccountSetPlanBillingRequestBuilder::plan)
    pub fn build(self) -> Result<AccountSetPlanBillingRequest, BuildError> {
        Ok(AccountSetPlanBillingRequest {
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
        })
    }
}
