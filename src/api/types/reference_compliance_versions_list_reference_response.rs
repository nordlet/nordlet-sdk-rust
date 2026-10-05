pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComplianceVersionsListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<ComplianceVersionsListReferenceResponseRowsItem>,
}

impl ComplianceVersionsListReferenceResponse {
    pub fn builder() -> ComplianceVersionsListReferenceResponseBuilder {
        <ComplianceVersionsListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComplianceVersionsListReferenceResponseBuilder {
    rows: Option<Vec<ComplianceVersionsListReferenceResponseRowsItem>>,
}

impl ComplianceVersionsListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<ComplianceVersionsListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComplianceVersionsListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ComplianceVersionsListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<ComplianceVersionsListReferenceResponse, BuildError> {
        Ok(ComplianceVersionsListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
