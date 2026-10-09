pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessTripsGetHrRequest {
    #[serde(default)]
    pub id: String,
}

impl BusinessTripsGetHrRequest {
    pub fn builder() -> BusinessTripsGetHrRequestBuilder {
        <BusinessTripsGetHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsGetHrRequestBuilder {
    id: Option<String>,
}

impl BusinessTripsGetHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsGetHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BusinessTripsGetHrRequestBuilder::id)
    pub fn build(self) -> Result<BusinessTripsGetHrRequest, BuildError> {
        Ok(BusinessTripsGetHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
