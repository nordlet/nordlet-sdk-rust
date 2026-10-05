pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CnCodesUpsertReferenceRequest {
    #[serde(default)]
    pub rows: Vec<CnCodesUpsertReferenceRequestRowsItem>,
}

impl CnCodesUpsertReferenceRequest {
    pub fn builder() -> CnCodesUpsertReferenceRequestBuilder {
        <CnCodesUpsertReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CnCodesUpsertReferenceRequestBuilder {
    rows: Option<Vec<CnCodesUpsertReferenceRequestRowsItem>>,
}

impl CnCodesUpsertReferenceRequestBuilder {
    pub fn rows(mut self, value: Vec<CnCodesUpsertReferenceRequestRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CnCodesUpsertReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](CnCodesUpsertReferenceRequestBuilder::rows)
    pub fn build(self) -> Result<CnCodesUpsertReferenceRequest, BuildError> {
        Ok(CnCodesUpsertReferenceRequest {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
