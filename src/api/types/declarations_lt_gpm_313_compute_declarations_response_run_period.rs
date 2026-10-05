pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm313ComputeDeclarationsResponseRunPeriod {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl LtGpm313ComputeDeclarationsResponseRunPeriod {
    pub fn builder() -> LtGpm313ComputeDeclarationsResponseRunPeriodBuilder {
        <LtGpm313ComputeDeclarationsResponseRunPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm313ComputeDeclarationsResponseRunPeriodBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl LtGpm313ComputeDeclarationsResponseRunPeriodBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtGpm313ComputeDeclarationsResponseRunPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtGpm313ComputeDeclarationsResponseRunPeriodBuilder::year)
    /// - [`month`](LtGpm313ComputeDeclarationsResponseRunPeriodBuilder::month)
    pub fn build(self) -> Result<LtGpm313ComputeDeclarationsResponseRunPeriod, BuildError> {
        Ok(LtGpm313ComputeDeclarationsResponseRunPeriod {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
