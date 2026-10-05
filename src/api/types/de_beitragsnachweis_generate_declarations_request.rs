pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeBeitragsnachweisGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl DeBeitragsnachweisGenerateDeclarationsRequest {
    pub fn builder() -> DeBeitragsnachweisGenerateDeclarationsRequestBuilder {
        <DeBeitragsnachweisGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeBeitragsnachweisGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl DeBeitragsnachweisGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeBeitragsnachweisGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeBeitragsnachweisGenerateDeclarationsRequestBuilder::year)
    /// - [`month`](DeBeitragsnachweisGenerateDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<DeBeitragsnachweisGenerateDeclarationsRequest, BuildError> {
        Ok(DeBeitragsnachweisGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
