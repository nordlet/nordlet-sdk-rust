pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm312ComputeDeclarationsResponsePayoutFrom {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl LtGpm312ComputeDeclarationsResponsePayoutFrom {
    pub fn builder() -> LtGpm312ComputeDeclarationsResponsePayoutFromBuilder {
        <LtGpm312ComputeDeclarationsResponsePayoutFromBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm312ComputeDeclarationsResponsePayoutFromBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl LtGpm312ComputeDeclarationsResponsePayoutFromBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtGpm312ComputeDeclarationsResponsePayoutFrom`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtGpm312ComputeDeclarationsResponsePayoutFromBuilder::year)
    /// - [`month`](LtGpm312ComputeDeclarationsResponsePayoutFromBuilder::month)
    pub fn build(self) -> Result<LtGpm312ComputeDeclarationsResponsePayoutFrom, BuildError> {
        Ok(LtGpm312ComputeDeclarationsResponsePayoutFrom {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
