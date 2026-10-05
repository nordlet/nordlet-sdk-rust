pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatClassifiersUpsertReferenceRequest {
    #[serde(default)]
    pub rows: Vec<VatClassifiersUpsertReferenceRequestRowsItem>,
}

impl VatClassifiersUpsertReferenceRequest {
    pub fn builder() -> VatClassifiersUpsertReferenceRequestBuilder {
        <VatClassifiersUpsertReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersUpsertReferenceRequestBuilder {
    rows: Option<Vec<VatClassifiersUpsertReferenceRequestRowsItem>>,
}

impl VatClassifiersUpsertReferenceRequestBuilder {
    pub fn rows(mut self, value: Vec<VatClassifiersUpsertReferenceRequestRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatClassifiersUpsertReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](VatClassifiersUpsertReferenceRequestBuilder::rows)
    pub fn build(self) -> Result<VatClassifiersUpsertReferenceRequest, BuildError> {
        Ok(VatClassifiersUpsertReferenceRequest {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
