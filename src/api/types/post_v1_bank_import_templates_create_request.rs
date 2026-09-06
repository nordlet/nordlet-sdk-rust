pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesCreateRequest {
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1BankImportTemplatesCreateRequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<PostV1BankImportTemplatesCreateRequestFieldsItem>>,
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
}

impl PostV1BankImportTemplatesCreateRequest {
    pub fn builder() -> PostV1BankImportTemplatesCreateRequestBuilder {
        <PostV1BankImportTemplatesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesCreateRequestBuilder {
    name: Option<String>,
    r#type: Option<PostV1BankImportTemplatesCreateRequestType>,
    fields: Option<Vec<PostV1BankImportTemplatesCreateRequestFieldsItem>>,
    meta_fields: Option<Vec<String>>,
    invoice_meta_field: Option<String>,
    invoice_vat_rate_percent: Option<String>,
    company_meta_field: Option<String>,
    invoice_item_id: Option<String>,
    advance_invoices: Option<bool>,
}

impl PostV1BankImportTemplatesCreateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1BankImportTemplatesCreateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<PostV1BankImportTemplatesCreateRequestFieldsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1BankImportTemplatesCreateRequestBuilder::name)
    /// - [`r#type`](PostV1BankImportTemplatesCreateRequestBuilder::r#type)
    pub fn build(self) -> Result<PostV1BankImportTemplatesCreateRequest, BuildError> {
        Ok(PostV1BankImportTemplatesCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            fields: self.fields,
            meta_fields: self.meta_fields,
            invoice_meta_field: self.invoice_meta_field,
            invoice_vat_rate_percent: self.invoice_vat_rate_percent,
            company_meta_field: self.company_meta_field,
            invoice_item_id: self.invoice_item_id,
            advance_invoices: self.advance_invoices,
        })
    }
}
