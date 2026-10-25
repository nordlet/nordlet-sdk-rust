pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesGetPublicResponse {
    pub r#type: InvoicesGetPublicResponseType,
    pub status: InvoicesGetPublicResponseStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: InvoicesGetPublicResponsePaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "salesTaxPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tax_percent: Option<String>,
    #[serde(rename = "salesTaxTotal")]
    #[serde(default)]
    pub sales_tax_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(rename = "paidOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "payUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
    #[serde(default)]
    pub seller: InvoicesGetPublicResponseSeller,
    #[serde(default)]
    pub buyer: InvoicesGetPublicResponseBuyer,
    #[serde(default)]
    pub lines: Vec<InvoicesGetPublicResponseLinesItem>,
}

impl InvoicesGetPublicResponse {
    pub fn builder() -> InvoicesGetPublicResponseBuilder {
        <InvoicesGetPublicResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetPublicResponseBuilder {
    r#type: Option<InvoicesGetPublicResponseType>,
    status: Option<InvoicesGetPublicResponseStatus>,
    payment_status: Option<InvoicesGetPublicResponsePaymentStatus>,
    number: Option<String>,
    issue_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    sales_tax_percent: Option<String>,
    sales_tax_total: Option<String>,
    gross_total: Option<String>,
    paid_amount: Option<String>,
    paid_on: Option<String>,
    notes: Option<String>,
    pay_url: Option<String>,
    seller: Option<InvoicesGetPublicResponseSeller>,
    buyer: Option<InvoicesGetPublicResponseBuyer>,
    lines: Option<Vec<InvoicesGetPublicResponseLinesItem>>,
}

impl InvoicesGetPublicResponseBuilder {
    pub fn r#type(mut self, value: InvoicesGetPublicResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: InvoicesGetPublicResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(mut self, value: InvoicesGetPublicResponsePaymentStatus) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn sales_tax_percent(mut self, value: impl Into<String>) -> Self {
        self.sales_tax_percent = Some(value.into());
        self
    }

    pub fn sales_tax_total(mut self, value: impl Into<String>) -> Self {
        self.sales_tax_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
        self
    }

    pub fn paid_on(mut self, value: impl Into<String>) -> Self {
        self.paid_on = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn pay_url(mut self, value: impl Into<String>) -> Self {
        self.pay_url = Some(value.into());
        self
    }

    pub fn seller(mut self, value: InvoicesGetPublicResponseSeller) -> Self {
        self.seller = Some(value);
        self
    }

    pub fn buyer(mut self, value: InvoicesGetPublicResponseBuyer) -> Self {
        self.buyer = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<InvoicesGetPublicResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetPublicResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InvoicesGetPublicResponseBuilder::r#type)
    /// - [`status`](InvoicesGetPublicResponseBuilder::status)
    /// - [`payment_status`](InvoicesGetPublicResponseBuilder::payment_status)
    /// - [`currency`](InvoicesGetPublicResponseBuilder::currency)
    /// - [`net_total`](InvoicesGetPublicResponseBuilder::net_total)
    /// - [`vat_total`](InvoicesGetPublicResponseBuilder::vat_total)
    /// - [`sales_tax_total`](InvoicesGetPublicResponseBuilder::sales_tax_total)
    /// - [`gross_total`](InvoicesGetPublicResponseBuilder::gross_total)
    /// - [`paid_amount`](InvoicesGetPublicResponseBuilder::paid_amount)
    /// - [`seller`](InvoicesGetPublicResponseBuilder::seller)
    /// - [`buyer`](InvoicesGetPublicResponseBuilder::buyer)
    /// - [`lines`](InvoicesGetPublicResponseBuilder::lines)
    pub fn build(self) -> Result<InvoicesGetPublicResponse, BuildError> {
        Ok(InvoicesGetPublicResponse {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            payment_status: self
                .payment_status
                .ok_or_else(|| BuildError::missing_field("payment_status"))?,
            number: self.number,
            issue_date: self.issue_date,
            due_date: self.due_date,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            sales_tax_percent: self.sales_tax_percent,
            sales_tax_total: self
                .sales_tax_total
                .ok_or_else(|| BuildError::missing_field("sales_tax_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            paid_on: self.paid_on,
            notes: self.notes,
            pay_url: self.pay_url,
            seller: self
                .seller
                .ok_or_else(|| BuildError::missing_field("seller"))?,
            buyer: self
                .buyer
                .ok_or_else(|| BuildError::missing_field("buyer"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
