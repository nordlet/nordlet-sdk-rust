pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlKsefReceiptDeclarationsRequest {
    #[serde(rename = "sessionReferenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_reference_number: Option<String>,
}

impl PlKsefReceiptDeclarationsRequest {
    pub fn builder() -> PlKsefReceiptDeclarationsRequestBuilder {
        <PlKsefReceiptDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlKsefReceiptDeclarationsRequestBuilder {
    session_reference_number: Option<String>,
}

impl PlKsefReceiptDeclarationsRequestBuilder {
    pub fn session_reference_number(mut self, value: impl Into<String>) -> Self {
        self.session_reference_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlKsefReceiptDeclarationsRequest`].
    pub fn build(self) -> Result<PlKsefReceiptDeclarationsRequest, BuildError> {
        Ok(PlKsefReceiptDeclarationsRequest {
            session_reference_number: self.session_reference_number,
        })
    }
}
