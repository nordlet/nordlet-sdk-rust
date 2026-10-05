pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazGenerateDeclarationsResponseCounts {
    #[serde(default)]
    pub documents: i64,
}

impl LtIvazGenerateDeclarationsResponseCounts {
    pub fn builder() -> LtIvazGenerateDeclarationsResponseCountsBuilder {
        <LtIvazGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazGenerateDeclarationsResponseCountsBuilder {
    documents: Option<i64>,
}

impl LtIvazGenerateDeclarationsResponseCountsBuilder {
    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIvazGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](LtIvazGenerateDeclarationsResponseCountsBuilder::documents)
    pub fn build(self) -> Result<LtIvazGenerateDeclarationsResponseCounts, BuildError> {
        Ok(LtIvazGenerateDeclarationsResponseCounts {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
