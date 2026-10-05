pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDistanceSalesThresholdGetDeclarationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl EuDistanceSalesThresholdGetDeclarationsRequest {
    pub fn builder() -> EuDistanceSalesThresholdGetDeclarationsRequestBuilder {
        <EuDistanceSalesThresholdGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDistanceSalesThresholdGetDeclarationsRequestBuilder {
    date: Option<NaiveDate>,
}

impl EuDistanceSalesThresholdGetDeclarationsRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuDistanceSalesThresholdGetDeclarationsRequest`].
    pub fn build(self) -> Result<EuDistanceSalesThresholdGetDeclarationsRequest, BuildError> {
        Ok(EuDistanceSalesThresholdGetDeclarationsRequest { date: self.date })
    }
}
