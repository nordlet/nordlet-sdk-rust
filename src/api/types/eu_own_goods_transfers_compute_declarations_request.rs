pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOwnGoodsTransfersComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl EuOwnGoodsTransfersComputeDeclarationsRequest {
    pub fn builder() -> EuOwnGoodsTransfersComputeDeclarationsRequestBuilder {
        <EuOwnGoodsTransfersComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOwnGoodsTransfersComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl EuOwnGoodsTransfersComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuOwnGoodsTransfersComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuOwnGoodsTransfersComputeDeclarationsRequestBuilder::year)
    /// - [`month`](EuOwnGoodsTransfersComputeDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<EuOwnGoodsTransfersComputeDeclarationsRequest, BuildError> {
        Ok(EuOwnGoodsTransfersComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
