pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtAgingReportsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<DebtAgingReportsRequestSide>,
    #[serde(rename = "asOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<NaiveDate>,
}

impl DebtAgingReportsRequest {
    pub fn builder() -> DebtAgingReportsRequestBuilder {
        <DebtAgingReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtAgingReportsRequestBuilder {
    side: Option<DebtAgingReportsRequestSide>,
    as_of: Option<NaiveDate>,
}

impl DebtAgingReportsRequestBuilder {
    pub fn side(mut self, value: DebtAgingReportsRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn as_of(mut self, value: NaiveDate) -> Self {
        self.as_of = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtAgingReportsRequest`].
    pub fn build(self) -> Result<DebtAgingReportsRequest, BuildError> {
        Ok(DebtAgingReportsRequest {
            side: self.side,
            as_of: self.as_of,
        })
    }
}
