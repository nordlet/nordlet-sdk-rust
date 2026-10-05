pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsageListBillingRequest {
    #[serde(default)]
    pub from: NaiveDate,
    #[serde(default)]
    pub to: NaiveDate,
}

impl UsageListBillingRequest {
    pub fn builder() -> UsageListBillingRequestBuilder {
        <UsageListBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageListBillingRequestBuilder {
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
}

impl UsageListBillingRequestBuilder {
    pub fn from(mut self, value: NaiveDate) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: NaiveDate) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageListBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsageListBillingRequestBuilder::from)
    /// - [`to`](UsageListBillingRequestBuilder::to)
    pub fn build(self) -> Result<UsageListBillingRequest, BuildError> {
        Ok(UsageListBillingRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
