pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaybillsCancelTransportRequest {
    #[serde(default)]
    pub id: String,
}

impl WaybillsCancelTransportRequest {
    pub fn builder() -> WaybillsCancelTransportRequestBuilder {
        <WaybillsCancelTransportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsCancelTransportRequestBuilder {
    id: Option<String>,
}

impl WaybillsCancelTransportRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WaybillsCancelTransportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaybillsCancelTransportRequestBuilder::id)
    pub fn build(self) -> Result<WaybillsCancelTransportRequest, BuildError> {
        Ok(WaybillsCancelTransportRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
