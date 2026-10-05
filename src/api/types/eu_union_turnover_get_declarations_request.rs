pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuUnionTurnoverGetDeclarationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl EuUnionTurnoverGetDeclarationsRequest {
    pub fn builder() -> EuUnionTurnoverGetDeclarationsRequestBuilder {
        <EuUnionTurnoverGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuUnionTurnoverGetDeclarationsRequestBuilder {
    date: Option<NaiveDate>,
}

impl EuUnionTurnoverGetDeclarationsRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuUnionTurnoverGetDeclarationsRequest`].
    pub fn build(self) -> Result<EuUnionTurnoverGetDeclarationsRequest, BuildError> {
        Ok(EuUnionTurnoverGetDeclarationsRequest { date: self.date })
    }
}
