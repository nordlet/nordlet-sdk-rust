pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazAmendDeclarationsResponseCounts {
    #[serde(default)]
    pub documents: i64,
}

impl LtIvazAmendDeclarationsResponseCounts {
    pub fn builder() -> LtIvazAmendDeclarationsResponseCountsBuilder {
        <LtIvazAmendDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazAmendDeclarationsResponseCountsBuilder {
    documents: Option<i64>,
}

impl LtIvazAmendDeclarationsResponseCountsBuilder {
    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIvazAmendDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](LtIvazAmendDeclarationsResponseCountsBuilder::documents)
    pub fn build(self) -> Result<LtIvazAmendDeclarationsResponseCounts, BuildError> {
        Ok(LtIvazAmendDeclarationsResponseCounts {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
