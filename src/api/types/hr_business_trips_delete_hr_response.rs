pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessTripsDeleteHrResponse {
    #[serde(default)]
    pub id: String,
}

impl BusinessTripsDeleteHrResponse {
    pub fn builder() -> BusinessTripsDeleteHrResponseBuilder {
        <BusinessTripsDeleteHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsDeleteHrResponseBuilder {
    id: Option<String>,
}

impl BusinessTripsDeleteHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsDeleteHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BusinessTripsDeleteHrResponseBuilder::id)
    pub fn build(self) -> Result<BusinessTripsDeleteHrResponse, BuildError> {
        Ok(BusinessTripsDeleteHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
