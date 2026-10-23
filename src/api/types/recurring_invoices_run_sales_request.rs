pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesRunSalesRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<NaiveDate>,
}

impl RecurringInvoicesRunSalesRequest {
    pub fn builder() -> RecurringInvoicesRunSalesRequestBuilder {
        <RecurringInvoicesRunSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesRunSalesRequestBuilder {
    as_of_date: Option<NaiveDate>,
}

impl RecurringInvoicesRunSalesRequestBuilder {
    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesRunSalesRequest`].
    pub fn build(self) -> Result<RecurringInvoicesRunSalesRequest, BuildError> {
        Ok(RecurringInvoicesRunSalesRequest {
            as_of_date: self.as_of_date,
        })
    }
}
