pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatClassifiersUpsertReferenceResponse {
    #[serde(default)]
    pub upserted: i64,
}

impl VatClassifiersUpsertReferenceResponse {
    pub fn builder() -> VatClassifiersUpsertReferenceResponseBuilder {
        <VatClassifiersUpsertReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersUpsertReferenceResponseBuilder {
    upserted: Option<i64>,
}

impl VatClassifiersUpsertReferenceResponseBuilder {
    pub fn upserted(mut self, value: i64) -> Self {
        self.upserted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatClassifiersUpsertReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`upserted`](VatClassifiersUpsertReferenceResponseBuilder::upserted)
    pub fn build(self) -> Result<VatClassifiersUpsertReferenceResponse, BuildError> {
        Ok(VatClassifiersUpsertReferenceResponse {
            upserted: self
                .upserted
                .ok_or_else(|| BuildError::missing_field("upserted"))?,
        })
    }
}
