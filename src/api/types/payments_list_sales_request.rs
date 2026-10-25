pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentsListSalesRequest {
    #[serde(default)]
    pub from: NaiveDate,
    #[serde(default)]
    pub to: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl PaymentsListSalesRequest {
    pub fn builder() -> PaymentsListSalesRequestBuilder {
        <PaymentsListSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListSalesRequestBuilder {
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl PaymentsListSalesRequestBuilder {
    pub fn from(mut self, value: NaiveDate) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: NaiveDate) -> Self {
        self.to = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsListSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](PaymentsListSalesRequestBuilder::from)
    /// - [`to`](PaymentsListSalesRequestBuilder::to)
    pub fn build(self) -> Result<PaymentsListSalesRequest, BuildError> {
        Ok(PaymentsListSalesRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            page: self.page,
            page_size: self.page_size,
        })
    }
}
