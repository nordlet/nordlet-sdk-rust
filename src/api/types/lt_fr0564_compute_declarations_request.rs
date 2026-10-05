pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0564ComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl LtFr0564ComputeDeclarationsRequest {
    pub fn builder() -> LtFr0564ComputeDeclarationsRequestBuilder {
        <LtFr0564ComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0564ComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl LtFr0564ComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtFr0564ComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtFr0564ComputeDeclarationsRequestBuilder::year)
    /// - [`month`](LtFr0564ComputeDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<LtFr0564ComputeDeclarationsRequest, BuildError> {
        Ok(LtFr0564ComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
