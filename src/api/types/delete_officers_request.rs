pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteOfficersRequest {
    #[serde(default)]
    pub id: String,
}

impl DeleteOfficersRequest {
    pub fn builder() -> DeleteOfficersRequestBuilder {
        <DeleteOfficersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteOfficersRequestBuilder {
    id: Option<String>,
}

impl DeleteOfficersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteOfficersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteOfficersRequestBuilder::id)
    pub fn build(self) -> Result<DeleteOfficersRequest, BuildError> {
        Ok(DeleteOfficersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
