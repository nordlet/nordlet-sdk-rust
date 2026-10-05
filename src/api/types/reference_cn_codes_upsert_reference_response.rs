pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CnCodesUpsertReferenceResponse {
    #[serde(default)]
    pub upserted: i64,
}

impl CnCodesUpsertReferenceResponse {
    pub fn builder() -> CnCodesUpsertReferenceResponseBuilder {
        <CnCodesUpsertReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CnCodesUpsertReferenceResponseBuilder {
    upserted: Option<i64>,
}

impl CnCodesUpsertReferenceResponseBuilder {
    pub fn upserted(mut self, value: i64) -> Self {
        self.upserted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CnCodesUpsertReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`upserted`](CnCodesUpsertReferenceResponseBuilder::upserted)
    pub fn build(self) -> Result<CnCodesUpsertReferenceResponse, BuildError> {
        Ok(CnCodesUpsertReferenceResponse {
            upserted: self
                .upserted
                .ok_or_else(|| BuildError::missing_field("upserted"))?,
        })
    }
}
