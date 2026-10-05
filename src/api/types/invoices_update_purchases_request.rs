pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoicesUpdatePurchasesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "documentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<NaiveDate>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "purchaseOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_id: Option<String>,
    #[serde(rename = "operationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "intrastatTransportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_transport_mode: Option<String>,
    #[serde(rename = "intrastatDeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_delivery_terms: Option<String>,
    #[serde(rename = "intrastatRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_region: Option<String>,
    #[serde(rename = "intrastatNatureOfTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_nature_of_transaction: Option<String>,
    #[serde(rename = "einvoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<InvoicesUpdatePurchasesRequestLinesItem>>,
}

impl InvoicesUpdatePurchasesRequest {
    pub fn builder() -> InvoicesUpdatePurchasesRequestBuilder {
        <InvoicesUpdatePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUpdatePurchasesRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    document_number: Option<String>,
    document_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    purchase_order_id: Option<String>,
    operation_type_id: Option<String>,
    notes: Option<String>,
    intrastat_transport_mode: Option<String>,
    intrastat_delivery_terms: Option<String>,
    intrastat_region: Option<String>,
    intrastat_nature_of_transaction: Option<String>,
    einvoice_number: Option<String>,
    lines: Option<Vec<InvoicesUpdatePurchasesRequestLinesItem>>,
}

impl InvoicesUpdatePurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: NaiveDate) -> Self {
        self.document_date = Some(value);
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

    pub fn purchase_order_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_id = Some(value.into());
        self
    }

    pub fn operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.operation_type_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn intrastat_transport_mode(mut self, value: impl Into<String>) -> Self {
        self.intrastat_transport_mode = Some(value.into());
        self
    }

    pub fn intrastat_delivery_terms(mut self, value: impl Into<String>) -> Self {
        self.intrastat_delivery_terms = Some(value.into());
        self
    }

    pub fn intrastat_region(mut self, value: impl Into<String>) -> Self {
        self.intrastat_region = Some(value.into());
        self
    }

    pub fn intrastat_nature_of_transaction(mut self, value: impl Into<String>) -> Self {
        self.intrastat_nature_of_transaction = Some(value.into());
        self
    }

    pub fn einvoice_number(mut self, value: impl Into<String>) -> Self {
        self.einvoice_number = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<InvoicesUpdatePurchasesRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesUpdatePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesUpdatePurchasesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesUpdatePurchasesRequest, BuildError> {
        Ok(InvoicesUpdatePurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            document_number: self.document_number,
            document_date: self.document_date,
            due_date: self.due_date,
            currency: self.currency,
            purchase_order_id: self.purchase_order_id,
            operation_type_id: self.operation_type_id,
            notes: self.notes,
            intrastat_transport_mode: self.intrastat_transport_mode,
            intrastat_delivery_terms: self.intrastat_delivery_terms,
            intrastat_region: self.intrastat_region,
            intrastat_nature_of_transaction: self.intrastat_nature_of_transaction,
            einvoice_number: self.einvoice_number,
            lines: self.lines,
        })
    }
}
