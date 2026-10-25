pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentsListSalesResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    pub method: PaymentsListSalesResponseRowsItemMethod,
}

impl PaymentsListSalesResponseRowsItem {
    pub fn builder() -> PaymentsListSalesResponseRowsItemBuilder {
        <PaymentsListSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListSalesResponseRowsItemBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    invoice_id: Option<String>,
    full_number: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    amount: Option<String>,
    currency: Option<String>,
    method: Option<PaymentsListSalesResponseRowsItemMethod>,
}

impl PaymentsListSalesResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn method(mut self, value: PaymentsListSalesResponseRowsItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsListSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentsListSalesResponseRowsItemBuilder::id)
    /// - [`date`](PaymentsListSalesResponseRowsItemBuilder::date)
    /// - [`invoice_id`](PaymentsListSalesResponseRowsItemBuilder::invoice_id)
    /// - [`partner_id`](PaymentsListSalesResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](PaymentsListSalesResponseRowsItemBuilder::partner_name)
    /// - [`amount`](PaymentsListSalesResponseRowsItemBuilder::amount)
    /// - [`currency`](PaymentsListSalesResponseRowsItemBuilder::currency)
    /// - [`method`](PaymentsListSalesResponseRowsItemBuilder::method)
    pub fn build(self) -> Result<PaymentsListSalesResponseRowsItem, BuildError> {
        Ok(PaymentsListSalesResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            full_number: self.full_number,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
