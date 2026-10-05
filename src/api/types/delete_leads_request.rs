pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteLeadsRequest {
    #[serde(default)]
    pub id: String,
}

impl DeleteLeadsRequest {
    pub fn builder() -> DeleteLeadsRequestBuilder {
        <DeleteLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteLeadsRequestBuilder {
    id: Option<String>,
}

impl DeleteLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteLeadsRequestBuilder::id)
    pub fn build(self) -> Result<DeleteLeadsRequest, BuildError> {
        Ok(DeleteLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
