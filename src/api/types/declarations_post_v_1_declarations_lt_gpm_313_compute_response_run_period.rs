pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm313ComputeResponseRunPeriod {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsLtGpm313ComputeResponseRunPeriod {
    pub fn builder() -> PostV1DeclarationsLtGpm313ComputeResponseRunPeriodBuilder {
        <PostV1DeclarationsLtGpm313ComputeResponseRunPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm313ComputeResponseRunPeriodBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsLtGpm313ComputeResponseRunPeriodBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm313ComputeResponseRunPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtGpm313ComputeResponseRunPeriodBuilder::year)
    /// - [`month`](PostV1DeclarationsLtGpm313ComputeResponseRunPeriodBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm313ComputeResponseRunPeriod, BuildError> {
        Ok(PostV1DeclarationsLtGpm313ComputeResponseRunPeriod {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
