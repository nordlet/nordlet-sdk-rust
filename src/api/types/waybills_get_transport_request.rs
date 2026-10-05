pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaybillsGetTransportRequest {
    #[serde(default)]
    pub id: String,
}

impl WaybillsGetTransportRequest {
    pub fn builder() -> WaybillsGetTransportRequestBuilder {
        <WaybillsGetTransportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsGetTransportRequestBuilder {
    id: Option<String>,
}

impl WaybillsGetTransportRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WaybillsGetTransportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaybillsGetTransportRequestBuilder::id)
    pub fn build(self) -> Result<WaybillsGetTransportRequest, BuildError> {
        Ok(WaybillsGetTransportRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
