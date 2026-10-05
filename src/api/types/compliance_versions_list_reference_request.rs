pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComplianceVersionsListReferenceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl ComplianceVersionsListReferenceRequest {
    pub fn builder() -> ComplianceVersionsListReferenceRequestBuilder {
        <ComplianceVersionsListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComplianceVersionsListReferenceRequestBuilder {
    country: Option<String>,
}

impl ComplianceVersionsListReferenceRequestBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ComplianceVersionsListReferenceRequest`].
    pub fn build(self) -> Result<ComplianceVersionsListReferenceRequest, BuildError> {
        Ok(ComplianceVersionsListReferenceRequest {
            country: self.country,
        })
    }
}
