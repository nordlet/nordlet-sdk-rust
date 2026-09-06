pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1BankImportTemplatesUpdateRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<PostV1BankImportTemplatesUpdateRequestFieldsItem>>,
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
    #[serde(default)]
    pub id: String,
}

impl PostV1BankImportTemplatesUpdateRequest {
    pub fn builder() -> PostV1BankImportTemplatesUpdateRequestBuilder {
        <PostV1BankImportTemplatesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesUpdateRequestBuilder {
    name: Option<String>,
    r#type: Option<PostV1BankImportTemplatesUpdateRequestType>,
    fields: Option<Vec<PostV1BankImportTemplatesUpdateRequestFieldsItem>>,
    meta_fields: Option<Vec<String>>,
    invoice_meta_field: Option<String>,
    invoice_vat_rate_percent: Option<String>,
    company_meta_field: Option<String>,
    invoice_item_id: Option<String>,
    advance_invoices: Option<bool>,
    id: Option<String>,
}

impl PostV1BankImportTemplatesUpdateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1BankImportTemplatesUpdateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<PostV1BankImportTemplatesUpdateRequestFieldsItem>) -> Self {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankImportTemplatesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankImportTemplatesUpdateRequest, BuildError> {
        Ok(PostV1BankImportTemplatesUpdateRequest {
            name: self.name,
            r#type: self.r#type,
            fields: self.fields,
            meta_fields: self.meta_fields,
            invoice_meta_field: self.invoice_meta_field,
            invoice_vat_rate_percent: self.invoice_vat_rate_percent,
            company_meta_field: self.company_meta_field,
            invoice_item_id: self.invoice_item_id,
            advance_invoices: self.advance_invoices,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
