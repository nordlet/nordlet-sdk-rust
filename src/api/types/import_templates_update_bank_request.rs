pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesUpdateBankRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ImportTemplatesUpdateBankRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ImportTemplatesUpdateBankRequestFieldsItem>>,
    #[serde(rename = "metaFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_fields: Option<Vec<String>>,
    #[serde(rename = "invoiceMetaField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_meta_field: Option<String>,
    #[serde(rename = "invoiceVatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_vat_rate_percent: Option<String>,
    #[serde(rename = "companyMetaField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_meta_field: Option<String>,
    #[serde(rename = "invoiceItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item_id: Option<String>,
    #[serde(rename = "advanceInvoices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_invoices: Option<bool>,
    #[serde(rename = "authorizationOperationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_operation_type_id: Option<String>,
    #[serde(rename = "payoutOperationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_operation_type_id: Option<String>,
    #[serde(rename = "commissionOperationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_operation_type_id: Option<String>,
    #[serde(rename = "lenderMetaField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lender_meta_field: Option<String>,
    #[serde(rename = "partialRefundLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_refund_label: Option<String>,
    #[serde(rename = "fullRefundLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refund_label: Option<String>,
    #[serde(default)]
    pub id: String,
}

impl ImportTemplatesUpdateBankRequest {
    pub fn builder() -> ImportTemplatesUpdateBankRequestBuilder {
        <ImportTemplatesUpdateBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesUpdateBankRequestBuilder {
    name: Option<String>,
    r#type: Option<ImportTemplatesUpdateBankRequestType>,
    fields: Option<Vec<ImportTemplatesUpdateBankRequestFieldsItem>>,
    meta_fields: Option<Vec<String>>,
    invoice_meta_field: Option<String>,
    invoice_vat_rate_percent: Option<String>,
    company_meta_field: Option<String>,
    invoice_item_id: Option<String>,
    advance_invoices: Option<bool>,
    authorization_operation_type_id: Option<String>,
    payout_operation_type_id: Option<String>,
    commission_operation_type_id: Option<String>,
    lender_meta_field: Option<String>,
    partial_refund_label: Option<String>,
    full_refund_label: Option<String>,
    id: Option<String>,
}

impl ImportTemplatesUpdateBankRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ImportTemplatesUpdateBankRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<ImportTemplatesUpdateBankRequestFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn meta_fields(mut self, value: Vec<String>) -> Self {
        self.meta_fields = Some(value);
        self
    }

    pub fn invoice_meta_field(mut self, value: impl Into<String>) -> Self {
        self.invoice_meta_field = Some(value.into());
        self
    }

    pub fn invoice_vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.invoice_vat_rate_percent = Some(value.into());
        self
    }

    pub fn company_meta_field(mut self, value: impl Into<String>) -> Self {
        self.company_meta_field = Some(value.into());
        self
    }

    pub fn invoice_item_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_item_id = Some(value.into());
        self
    }

    pub fn advance_invoices(mut self, value: bool) -> Self {
        self.advance_invoices = Some(value);
        self
    }

    pub fn authorization_operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.authorization_operation_type_id = Some(value.into());
        self
    }

    pub fn payout_operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.payout_operation_type_id = Some(value.into());
        self
    }

    pub fn commission_operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.commission_operation_type_id = Some(value.into());
        self
    }

    pub fn lender_meta_field(mut self, value: impl Into<String>) -> Self {
        self.lender_meta_field = Some(value.into());
        self
    }

    pub fn partial_refund_label(mut self, value: impl Into<String>) -> Self {
        self.partial_refund_label = Some(value.into());
        self
    }

    pub fn full_refund_label(mut self, value: impl Into<String>) -> Self {
        self.full_refund_label = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesUpdateBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ImportTemplatesUpdateBankRequestBuilder::id)
    pub fn build(self) -> Result<ImportTemplatesUpdateBankRequest, BuildError> {
        Ok(ImportTemplatesUpdateBankRequest {
            name: self.name,
            r#type: self.r#type,
            fields: self.fields,
            meta_fields: self.meta_fields,
            invoice_meta_field: self.invoice_meta_field,
            invoice_vat_rate_percent: self.invoice_vat_rate_percent,
            company_meta_field: self.company_meta_field,
            invoice_item_id: self.invoice_item_id,
            advance_invoices: self.advance_invoices,
            authorization_operation_type_id: self.authorization_operation_type_id,
            payout_operation_type_id: self.payout_operation_type_id,
            commission_operation_type_id: self.commission_operation_type_id,
            lender_meta_field: self.lender_meta_field,
            partial_refund_label: self.partial_refund_label,
            full_refund_label: self.full_refund_label,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
