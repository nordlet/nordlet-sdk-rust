pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOperationTypesRequest {
    #[serde(default)]
    pub id: String,
}

impl GetOperationTypesRequest {
    pub fn builder() -> GetOperationTypesRequestBuilder {
        <GetOperationTypesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOperationTypesRequestBuilder {
    id: Option<String>,
}

impl GetOperationTypesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOperationTypesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetOperationTypesRequestBuilder::id)
    pub fn build(self) -> Result<GetOperationTypesRequest, BuildError> {
        Ok(GetOperationTypesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
