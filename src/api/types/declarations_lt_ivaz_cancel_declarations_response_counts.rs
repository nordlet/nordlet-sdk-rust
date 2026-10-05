pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazCancelDeclarationsResponseCounts {
    #[serde(default)]
    pub documents: i64,
}

impl LtIvazCancelDeclarationsResponseCounts {
    pub fn builder() -> LtIvazCancelDeclarationsResponseCountsBuilder {
        <LtIvazCancelDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazCancelDeclarationsResponseCountsBuilder {
    documents: Option<i64>,
}

impl LtIvazCancelDeclarationsResponseCountsBuilder {
    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIvazCancelDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](LtIvazCancelDeclarationsResponseCountsBuilder::documents)
    pub fn build(self) -> Result<LtIvazCancelDeclarationsResponseCounts, BuildError> {
        Ok(LtIvazCancelDeclarationsResponseCounts {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
