pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsageListBillingResponse {
    #[serde(default)]
    pub rows: Vec<UsageListBillingResponseRowsItem>,
}

impl UsageListBillingResponse {
    pub fn builder() -> UsageListBillingResponseBuilder {
        <UsageListBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageListBillingResponseBuilder {
    rows: Option<Vec<UsageListBillingResponseRowsItem>>,
}

impl UsageListBillingResponseBuilder {
    pub fn rows(mut self, value: Vec<UsageListBillingResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageListBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](UsageListBillingResponseBuilder::rows)
    pub fn build(self) -> Result<UsageListBillingResponse, BuildError> {
        Ok(UsageListBillingResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
