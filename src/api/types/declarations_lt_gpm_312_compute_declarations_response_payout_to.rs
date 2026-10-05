pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm312ComputeDeclarationsResponsePayoutTo {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl LtGpm312ComputeDeclarationsResponsePayoutTo {
    pub fn builder() -> LtGpm312ComputeDeclarationsResponsePayoutToBuilder {
        <LtGpm312ComputeDeclarationsResponsePayoutToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm312ComputeDeclarationsResponsePayoutToBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl LtGpm312ComputeDeclarationsResponsePayoutToBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtGpm312ComputeDeclarationsResponsePayoutTo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtGpm312ComputeDeclarationsResponsePayoutToBuilder::year)
    /// - [`month`](LtGpm312ComputeDeclarationsResponsePayoutToBuilder::month)
    pub fn build(self) -> Result<LtGpm312ComputeDeclarationsResponsePayoutTo, BuildError> {
        Ok(LtGpm312ComputeDeclarationsResponsePayoutTo {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
