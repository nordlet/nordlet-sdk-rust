pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeferralsPostPurchasesRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<NaiveDate>,
}

impl DeferralsPostPurchasesRequest {
    pub fn builder() -> DeferralsPostPurchasesRequestBuilder {
        <DeferralsPostPurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeferralsPostPurchasesRequestBuilder {
    as_of_date: Option<NaiveDate>,
}

impl DeferralsPostPurchasesRequestBuilder {
    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeferralsPostPurchasesRequest`].
    pub fn build(self) -> Result<DeferralsPostPurchasesRequest, BuildError> {
        Ok(DeferralsPostPurchasesRequest {
            as_of_date: self.as_of_date,
        })
    }
}
