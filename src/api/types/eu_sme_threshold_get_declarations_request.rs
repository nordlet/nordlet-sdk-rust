pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdGetDeclarationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl EuSmeThresholdGetDeclarationsRequest {
    pub fn builder() -> EuSmeThresholdGetDeclarationsRequestBuilder {
        <EuSmeThresholdGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdGetDeclarationsRequestBuilder {
    date: Option<NaiveDate>,
}

impl EuSmeThresholdGetDeclarationsRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuSmeThresholdGetDeclarationsRequest`].
    pub fn build(self) -> Result<EuSmeThresholdGetDeclarationsRequest, BuildError> {
        Ok(EuSmeThresholdGetDeclarationsRequest { date: self.date })
    }
}
