pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtPln204ComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl LtPln204ComputeDeclarationsRequest {
    pub fn builder() -> LtPln204ComputeDeclarationsRequestBuilder {
        <LtPln204ComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204ComputeDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl LtPln204ComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtPln204ComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtPln204ComputeDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<LtPln204ComputeDeclarationsRequest, BuildError> {
        Ok(LtPln204ComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
