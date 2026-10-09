pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessTripsDeleteHrRequest {
    #[serde(default)]
    pub id: String,
}

impl BusinessTripsDeleteHrRequest {
    pub fn builder() -> BusinessTripsDeleteHrRequestBuilder {
        <BusinessTripsDeleteHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsDeleteHrRequestBuilder {
    id: Option<String>,
}

impl BusinessTripsDeleteHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsDeleteHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BusinessTripsDeleteHrRequestBuilder::id)
    pub fn build(self) -> Result<BusinessTripsDeleteHrRequest, BuildError> {
        Ok(BusinessTripsDeleteHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
