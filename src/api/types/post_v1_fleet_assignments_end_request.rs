pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetAssignmentsEndRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
}

impl PostV1FleetAssignmentsEndRequest {
    pub fn builder() -> PostV1FleetAssignmentsEndRequestBuilder {
        <PostV1FleetAssignmentsEndRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetAssignmentsEndRequestBuilder {
    id: Option<String>,
    to_date: Option<String>,
}

impl PostV1FleetAssignmentsEndRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetAssignmentsEndRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FleetAssignmentsEndRequestBuilder::id)
    /// - [`to_date`](PostV1FleetAssignmentsEndRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1FleetAssignmentsEndRequest, BuildError> {
        Ok(PostV1FleetAssignmentsEndRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
