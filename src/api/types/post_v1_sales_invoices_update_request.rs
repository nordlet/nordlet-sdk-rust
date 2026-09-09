pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1SalesInvoicesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<PostV1SalesInvoicesUpdateRequestVatScheme>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deemed_supplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "operationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type_id: Option<String>,
    #[serde(rename = "documentSeriesId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_series_id: Option<String>,
    #[serde(rename = "seriesLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_label: Option<String>,
    #[serde(rename = "discountPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<String>,
    #[serde(rename = "orderNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(rename = "issuedByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by_name: Option<String>,
    #[serde(rename = "issuedByTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by_title: Option<String>,
    #[serde(rename = "receivedByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_by_name: Option<String>,
    #[serde(rename = "receivedByTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_by_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItem>>,
}

impl PostV1SalesInvoicesUpdateRequest {
    pub fn builder() -> PostV1SalesInvoicesUpdateRequestBuilder {
        <PostV1SalesInvoicesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUpdateRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    currency: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    vat_scheme: Option<PostV1SalesInvoicesUpdateRequestVatScheme>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
    notes: Option<String>,
    operation_type_id: Option<String>,
    document_series_id: Option<String>,
    series_label: Option<String>,
    discount_percent: Option<String>,
    order_number: Option<String>,
    issued_by_name: Option<String>,
    issued_by_title: Option<String>,
    received_by_name: Option<String>,
    received_by_title: Option<String>,
    lines: Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItem>>,
}

impl PostV1SalesInvoicesUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    pub fn vat_scheme(mut self, value: PostV1SalesInvoicesUpdateRequestVatScheme) -> Self {
        self.vat_scheme = Some(value);
        self
    }

    pub fn vat_country_code(mut self, value: impl Into<String>) -> Self {
        self.vat_country_code = Some(value.into());
        self
    }

    pub fn deemed_supplier(mut self, value: bool) -> Self {
        self.deemed_supplier = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.operation_type_id = Some(value.into());
        self
    }

    pub fn document_series_id(mut self, value: impl Into<String>) -> Self {
        self.document_series_id = Some(value.into());
        self
    }

    pub fn series_label(mut self, value: impl Into<String>) -> Self {
        self.series_label = Some(value.into());
        self
    }

    pub fn discount_percent(mut self, value: impl Into<String>) -> Self {
        self.discount_percent = Some(value.into());
        self
    }

    pub fn order_number(mut self, value: impl Into<String>) -> Self {
        self.order_number = Some(value.into());
        self
    }

    pub fn issued_by_name(mut self, value: impl Into<String>) -> Self {
        self.issued_by_name = Some(value.into());
        self
    }

    pub fn issued_by_title(mut self, value: impl Into<String>) -> Self {
        self.issued_by_title = Some(value.into());
        self
    }

    pub fn received_by_name(mut self, value: impl Into<String>) -> Self {
        self.received_by_name = Some(value.into());
        self
    }

    pub fn received_by_title(mut self, value: impl Into<String>) -> Self {
        self.received_by_title = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1SalesInvoicesUpdateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesUpdateRequest, BuildError> {
        Ok(PostV1SalesInvoicesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            currency: self.currency,
            issue_date: self.issue_date,
            due_date: self.due_date,
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self.deemed_supplier,
            notes: self.notes,
            operation_type_id: self.operation_type_id,
            document_series_id: self.document_series_id,
            series_label: self.series_label,
            discount_percent: self.discount_percent,
            order_number: self.order_number,
            issued_by_name: self.issued_by_name,
            issued_by_title: self.issued_by_title,
            received_by_name: self.received_by_name,
            received_by_title: self.received_by_title,
            lines: self.lines,
        })
    }
}
