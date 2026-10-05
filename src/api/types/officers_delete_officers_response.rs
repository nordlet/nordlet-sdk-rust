pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteOfficersResponse {
    #[serde(default)]
    pub id: String,
}

impl DeleteOfficersResponse {
    pub fn builder() -> DeleteOfficersResponseBuilder {
        <DeleteOfficersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteOfficersResponseBuilder {
    id: Option<String>,
}

impl DeleteOfficersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteOfficersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteOfficersResponseBuilder::id)
    pub fn build(self) -> Result<DeleteOfficersResponse, BuildError> {
        Ok(DeleteOfficersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
