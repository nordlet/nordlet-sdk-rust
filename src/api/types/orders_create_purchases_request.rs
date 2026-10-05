pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrdersCreatePurchasesRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "orderNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(rename = "orderDate")]
    #[serde(default)]
    pub order_date: NaiveDate,
    #[serde(rename = "expectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<NaiveDate>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(default)]
    pub lines: Vec<OrdersCreatePurchasesRequestLinesItem>,
}

impl OrdersCreatePurchasesRequest {
    pub fn builder() -> OrdersCreatePurchasesRequestBuilder {
        <OrdersCreatePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCreatePurchasesRequestBuilder {
    partner_id: Option<String>,
    order_number: Option<String>,
    order_date: Option<NaiveDate>,
    expected_date: Option<NaiveDate>,
    warehouse_id: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    document_ref: Option<String>,
    lines: Option<Vec<OrdersCreatePurchasesRequestLinesItem>>,
}

impl OrdersCreatePurchasesRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn order_number(mut self, value: impl Into<String>) -> Self {
        self.order_number = Some(value.into());
        self
    }

    pub fn order_date(mut self, value: NaiveDate) -> Self {
        self.order_date = Some(value);
        self
    }

    pub fn expected_date(mut self, value: NaiveDate) -> Self {
        self.expected_date = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<OrdersCreatePurchasesRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersCreatePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](OrdersCreatePurchasesRequestBuilder::partner_id)
    /// - [`order_date`](OrdersCreatePurchasesRequestBuilder::order_date)
    /// - [`lines`](OrdersCreatePurchasesRequestBuilder::lines)
    pub fn build(self) -> Result<OrdersCreatePurchasesRequest, BuildError> {
        Ok(OrdersCreatePurchasesRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            order_number: self.order_number,
            order_date: self
                .order_date
                .ok_or_else(|| BuildError::missing_field("order_date"))?,
            expected_date: self.expected_date,
            warehouse_id: self.warehouse_id,
            currency: self.currency,
            notes: self.notes,
            document_ref: self.document_ref,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
