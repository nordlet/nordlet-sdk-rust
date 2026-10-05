pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoEtransportStatusDeclarationsRequest {
    #[serde(default)]
    pub reference: String,
}

impl RoEtransportStatusDeclarationsRequest {
    pub fn builder() -> RoEtransportStatusDeclarationsRequestBuilder {
        <RoEtransportStatusDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoEtransportStatusDeclarationsRequestBuilder {
    reference: Option<String>,
}

impl RoEtransportStatusDeclarationsRequestBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RoEtransportStatusDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](RoEtransportStatusDeclarationsRequestBuilder::reference)
    pub fn build(self) -> Result<RoEtransportStatusDeclarationsRequest, BuildError> {
        Ok(RoEtransportStatusDeclarationsRequest {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
