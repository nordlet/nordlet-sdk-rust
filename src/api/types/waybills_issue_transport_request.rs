pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaybillsIssueTransportRequest {
    #[serde(default)]
    pub id: String,
}

impl WaybillsIssueTransportRequest {
    pub fn builder() -> WaybillsIssueTransportRequestBuilder {
        <WaybillsIssueTransportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsIssueTransportRequestBuilder {
    id: Option<String>,
}

impl WaybillsIssueTransportRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WaybillsIssueTransportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaybillsIssueTransportRequestBuilder::id)
    pub fn build(self) -> Result<WaybillsIssueTransportRequest, BuildError> {
        Ok(WaybillsIssueTransportRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
