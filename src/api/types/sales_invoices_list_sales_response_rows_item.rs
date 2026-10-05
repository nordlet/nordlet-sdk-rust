pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesListSalesResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub r#type: InvoicesListSalesResponseRowsItemType,
    pub status: InvoicesListSalesResponseRowsItemStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: InvoicesListSalesResponseRowsItemPaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "fxRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_rate: Option<String>,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "appliedToInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_to_invoice_id: Option<String>,
    #[serde(rename = "creditedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_id: Option<String>,
    #[serde(rename = "creditedInvoiceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_reference: Option<String>,
    #[serde(rename = "creditedInvoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_date: Option<NaiveDate>,
    #[serde(rename = "agreementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<InvoicesListSalesResponseRowsItemVatScheme>,
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
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(default)]
    pub deemed_supplier: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
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
    #[serde(default)]
    pub discount_percent: String,
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
    #[serde(rename = "lockedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub locked_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "lockedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_by: Option<String>,
    #[serde(rename = "payToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_token: Option<String>,
    #[serde(rename = "einvoiceSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_system: Option<String>,
    #[serde(rename = "einvoiceTransport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_transport: Option<String>,
    #[serde(rename = "einvoiceMessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_message_id: Option<String>,
    #[serde(rename = "einvoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_number: Option<String>,
    #[serde(rename = "einvoiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_status: Option<String>,
    #[serde(rename = "einvoiceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_detail: Option<String>,
    #[serde(rename = "einvoiceSentAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub einvoice_sent_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "einvoiceCheckedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub einvoice_checked_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
}

impl InvoicesListSalesResponseRowsItem {
    pub fn builder() -> InvoicesListSalesResponseRowsItemBuilder {
        <InvoicesListSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesListSalesResponseRowsItemBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<InvoicesListSalesResponseRowsItemType>,
    status: Option<InvoicesListSalesResponseRowsItemStatus>,
    payment_status: Option<InvoicesListSalesResponseRowsItemPaymentStatus>,
    series: Option<String>,
    number: Option<i64>,
    full_number: Option<String>,
    issue_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    fx_rate: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    paid_amount: Option<String>,
    journal_transaction_id: Option<String>,
    applied_to_invoice_id: Option<String>,
    credited_invoice_id: Option<String>,
    credited_invoice_reference: Option<String>,
    credited_invoice_date: Option<NaiveDate>,
    agreement_id: Option<String>,
    vat_scheme: Option<InvoicesListSalesResponseRowsItemVatScheme>,
    intrastat_transport_mode: Option<String>,
    intrastat_delivery_terms: Option<String>,
    intrastat_region: Option<String>,
    intrastat_nature_of_transaction: Option<String>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
    notes: Option<String>,
    document_ref: Option<String>,
    operation_type_id: Option<String>,
    document_series_id: Option<String>,
    series_label: Option<String>,
    discount_percent: Option<String>,
    order_number: Option<String>,
    issued_by_name: Option<String>,
    issued_by_title: Option<String>,
    received_by_name: Option<String>,
    received_by_title: Option<String>,
    locked_at: Option<DateTime<FixedOffset>>,
    locked_by: Option<String>,
    pay_token: Option<String>,
    einvoice_system: Option<String>,
    einvoice_transport: Option<String>,
    einvoice_message_id: Option<String>,
    einvoice_number: Option<String>,
    einvoice_status: Option<String>,
    einvoice_detail: Option<String>,
    einvoice_sent_at: Option<DateTime<FixedOffset>>,
    einvoice_checked_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    partner_name: Option<String>,
}

impl InvoicesListSalesResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: InvoicesListSalesResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: InvoicesListSalesResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(mut self, value: InvoicesListSalesResponseRowsItemPaymentStatus) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
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

    pub fn fx_rate(mut self, value: impl Into<String>) -> Self {
        self.fx_rate = Some(value.into());
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

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn applied_to_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.applied_to_invoice_id = Some(value.into());
        self
    }

    pub fn credited_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_id = Some(value.into());
        self
    }

    pub fn credited_invoice_reference(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_reference = Some(value.into());
        self
    }

    pub fn credited_invoice_date(mut self, value: NaiveDate) -> Self {
        self.credited_invoice_date = Some(value);
        self
    }

    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn vat_scheme(mut self, value: InvoicesListSalesResponseRowsItemVatScheme) -> Self {
        self.vat_scheme = Some(value);
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

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
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

    pub fn locked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.locked_at = Some(value);
        self
    }

    pub fn locked_by(mut self, value: impl Into<String>) -> Self {
        self.locked_by = Some(value.into());
        self
    }

    pub fn pay_token(mut self, value: impl Into<String>) -> Self {
        self.pay_token = Some(value.into());
        self
    }

    pub fn einvoice_system(mut self, value: impl Into<String>) -> Self {
        self.einvoice_system = Some(value.into());
        self
    }

    pub fn einvoice_transport(mut self, value: impl Into<String>) -> Self {
        self.einvoice_transport = Some(value.into());
        self
    }

    pub fn einvoice_message_id(mut self, value: impl Into<String>) -> Self {
        self.einvoice_message_id = Some(value.into());
        self
    }

    pub fn einvoice_number(mut self, value: impl Into<String>) -> Self {
        self.einvoice_number = Some(value.into());
        self
    }

    pub fn einvoice_status(mut self, value: impl Into<String>) -> Self {
        self.einvoice_status = Some(value.into());
        self
    }

    pub fn einvoice_detail(mut self, value: impl Into<String>) -> Self {
        self.einvoice_detail = Some(value.into());
        self
    }

    pub fn einvoice_sent_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.einvoice_sent_at = Some(value);
        self
    }

    pub fn einvoice_checked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.einvoice_checked_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesListSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesListSalesResponseRowsItemBuilder::id)
    /// - [`partner_id`](InvoicesListSalesResponseRowsItemBuilder::partner_id)
    /// - [`r#type`](InvoicesListSalesResponseRowsItemBuilder::r#type)
    /// - [`status`](InvoicesListSalesResponseRowsItemBuilder::status)
    /// - [`payment_status`](InvoicesListSalesResponseRowsItemBuilder::payment_status)
    /// - [`currency`](InvoicesListSalesResponseRowsItemBuilder::currency)
    /// - [`net_total`](InvoicesListSalesResponseRowsItemBuilder::net_total)
    /// - [`vat_total`](InvoicesListSalesResponseRowsItemBuilder::vat_total)
    /// - [`gross_total`](InvoicesListSalesResponseRowsItemBuilder::gross_total)
    /// - [`paid_amount`](InvoicesListSalesResponseRowsItemBuilder::paid_amount)
    /// - [`deemed_supplier`](InvoicesListSalesResponseRowsItemBuilder::deemed_supplier)
    /// - [`discount_percent`](InvoicesListSalesResponseRowsItemBuilder::discount_percent)
    /// - [`created_at`](InvoicesListSalesResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](InvoicesListSalesResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<InvoicesListSalesResponseRowsItem, BuildError> {
        Ok(InvoicesListSalesResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            payment_status: self
                .payment_status
                .ok_or_else(|| BuildError::missing_field("payment_status"))?,
            series: self.series,
            number: self.number,
            full_number: self.full_number,
            issue_date: self.issue_date,
            due_date: self.due_date,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            fx_rate: self.fx_rate,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            journal_transaction_id: self.journal_transaction_id,
            applied_to_invoice_id: self.applied_to_invoice_id,
            credited_invoice_id: self.credited_invoice_id,
            credited_invoice_reference: self.credited_invoice_reference,
            credited_invoice_date: self.credited_invoice_date,
            agreement_id: self.agreement_id,
            vat_scheme: self.vat_scheme,
            intrastat_transport_mode: self.intrastat_transport_mode,
            intrastat_delivery_terms: self.intrastat_delivery_terms,
            intrastat_region: self.intrastat_region,
            intrastat_nature_of_transaction: self.intrastat_nature_of_transaction,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self
                .deemed_supplier
                .ok_or_else(|| BuildError::missing_field("deemed_supplier"))?,
            notes: self.notes,
            document_ref: self.document_ref,
            operation_type_id: self.operation_type_id,
            document_series_id: self.document_series_id,
            series_label: self.series_label,
            discount_percent: self
                .discount_percent
                .ok_or_else(|| BuildError::missing_field("discount_percent"))?,
            order_number: self.order_number,
            issued_by_name: self.issued_by_name,
            issued_by_title: self.issued_by_title,
            received_by_name: self.received_by_name,
            received_by_title: self.received_by_title,
            locked_at: self.locked_at,
            locked_by: self.locked_by,
            pay_token: self.pay_token,
            einvoice_system: self.einvoice_system,
            einvoice_transport: self.einvoice_transport,
            einvoice_message_id: self.einvoice_message_id,
            einvoice_number: self.einvoice_number,
            einvoice_status: self.einvoice_status,
            einvoice_detail: self.einvoice_detail,
            einvoice_sent_at: self.einvoice_sent_at,
            einvoice_checked_at: self.einvoice_checked_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            partner_name: self.partner_name,
        })
    }
}
