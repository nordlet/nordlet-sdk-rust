pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1BankImportTemplatesListResponseRowsItemType,
    #[serde(default)]
    pub fields: Vec<PostV1BankImportTemplatesListResponseRowsItemFieldsItem>,
    #[serde(rename = "metaFields")]
    #[serde(default)]
    pub meta_fields: Vec<String>,
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
    #[serde(default)]
    pub advance_invoices: bool,
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
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1BankImportTemplatesListResponseRowsItem {
    pub fn builder() -> PostV1BankImportTemplatesListResponseRowsItemBuilder {
        <PostV1BankImportTemplatesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesListResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1BankImportTemplatesListResponseRowsItemType>,
    fields: Option<Vec<PostV1BankImportTemplatesListResponseRowsItemFieldsItem>>,
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
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1BankImportTemplatesListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1BankImportTemplatesListResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: Vec<PostV1BankImportTemplatesListResponseRowsItemFieldsItem>,
    ) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankImportTemplatesListResponseRowsItemBuilder::id)
    /// - [`name`](PostV1BankImportTemplatesListResponseRowsItemBuilder::name)
    /// - [`r#type`](PostV1BankImportTemplatesListResponseRowsItemBuilder::r#type)
    /// - [`fields`](PostV1BankImportTemplatesListResponseRowsItemBuilder::fields)
    /// - [`meta_fields`](PostV1BankImportTemplatesListResponseRowsItemBuilder::meta_fields)
    /// - [`advance_invoices`](PostV1BankImportTemplatesListResponseRowsItemBuilder::advance_invoices)
    /// - [`created_at`](PostV1BankImportTemplatesListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1BankImportTemplatesListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1BankImportTemplatesListResponseRowsItem, BuildError> {
        Ok(PostV1BankImportTemplatesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            meta_fields: self
                .meta_fields
                .ok_or_else(|| BuildError::missing_field("meta_fields"))?,
            invoice_meta_field: self.invoice_meta_field,
            invoice_vat_rate_percent: self.invoice_vat_rate_percent,
            company_meta_field: self.company_meta_field,
            invoice_item_id: self.invoice_item_id,
            advance_invoices: self
                .advance_invoices
                .ok_or_else(|| BuildError::missing_field("advance_invoices"))?,
            authorization_operation_type_id: self.authorization_operation_type_id,
            payout_operation_type_id: self.payout_operation_type_id,
            commission_operation_type_id: self.commission_operation_type_id,
            lender_meta_field: self.lender_meta_field,
            partial_refund_label: self.partial_refund_label,
            full_refund_label: self.full_refund_label,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
