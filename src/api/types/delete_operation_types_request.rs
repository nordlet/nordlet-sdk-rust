pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteOperationTypesRequest {
    #[serde(default)]
    pub id: String,
}

impl DeleteOperationTypesRequest {
    pub fn builder() -> DeleteOperationTypesRequestBuilder {
        <DeleteOperationTypesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteOperationTypesRequestBuilder {
    id: Option<String>,
}

impl DeleteOperationTypesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteOperationTypesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteOperationTypesRequestBuilder::id)
    pub fn build(self) -> Result<DeleteOperationTypesRequest, BuildError> {
        Ok(DeleteOperationTypesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
