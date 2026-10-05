pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignmentsEndFleetRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl AssignmentsEndFleetRequest {
    pub fn builder() -> AssignmentsEndFleetRequestBuilder {
        <AssignmentsEndFleetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignmentsEndFleetRequestBuilder {
    id: Option<String>,
    to_date: Option<NaiveDate>,
}

impl AssignmentsEndFleetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssignmentsEndFleetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssignmentsEndFleetRequestBuilder::id)
    /// - [`to_date`](AssignmentsEndFleetRequestBuilder::to_date)
    pub fn build(self) -> Result<AssignmentsEndFleetRequest, BuildError> {
        Ok(AssignmentsEndFleetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
