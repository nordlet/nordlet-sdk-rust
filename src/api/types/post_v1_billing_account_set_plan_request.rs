pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BillingAccountSetPlanRequest {
    pub plan: PostV1BillingAccountSetPlanRequestPlan,
}

impl PostV1BillingAccountSetPlanRequest {
    pub fn builder() -> PostV1BillingAccountSetPlanRequestBuilder {
        <PostV1BillingAccountSetPlanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountSetPlanRequestBuilder {
    plan: Option<PostV1BillingAccountSetPlanRequestPlan>,
}

impl PostV1BillingAccountSetPlanRequestBuilder {
    pub fn plan(mut self, value: PostV1BillingAccountSetPlanRequestPlan) -> Self {
        self.plan = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingAccountSetPlanRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](PostV1BillingAccountSetPlanRequestBuilder::plan)
    pub fn build(self) -> Result<PostV1BillingAccountSetPlanRequest, BuildError> {
        Ok(PostV1BillingAccountSetPlanRequest {
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
        })
    }
}
