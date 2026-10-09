pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessTripsApproveHrRequest {
    #[serde(default)]
    pub id: String,
}

impl BusinessTripsApproveHrRequest {
    pub fn builder() -> BusinessTripsApproveHrRequestBuilder {
        <BusinessTripsApproveHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsApproveHrRequestBuilder {
    id: Option<String>,
}

impl BusinessTripsApproveHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsApproveHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BusinessTripsApproveHrRequestBuilder::id)
    pub fn build(self) -> Result<BusinessTripsApproveHrRequest, BuildError> {
        Ok(BusinessTripsApproveHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
