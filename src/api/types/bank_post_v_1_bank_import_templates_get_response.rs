pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub r#type: PostV1BankImportTemplatesGetResponseType,
    #[serde(default)]
    pub fields: Vec<PostV1BankImportTemplatesGetResponseFieldsItem>,
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
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1BankImportTemplatesGetResponse {
    pub fn builder() -> PostV1BankImportTemplatesGetResponseBuilder {
        <PostV1BankImportTemplatesGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesGetResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1BankImportTemplatesGetResponseType>,
    fields: Option<Vec<PostV1BankImportTemplatesGetResponseFieldsItem>>,
    meta_fields: Option<Vec<String>>,
    invoice_meta_field: Option<String>,
    invoice_vat_rate_percent: Option<String>,
    company_meta_field: Option<String>,
    invoice_item_id: Option<String>,
    advance_invoices: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1BankImportTemplatesGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1BankImportTemplatesGetResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<PostV1BankImportTemplatesGetResponseFieldsItem>) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankImportTemplatesGetResponseBuilder::id)
    /// - [`name`](PostV1BankImportTemplatesGetResponseBuilder::name)
    /// - [`r#type`](PostV1BankImportTemplatesGetResponseBuilder::r#type)
    /// - [`fields`](PostV1BankImportTemplatesGetResponseBuilder::fields)
    /// - [`meta_fields`](PostV1BankImportTemplatesGetResponseBuilder::meta_fields)
    /// - [`advance_invoices`](PostV1BankImportTemplatesGetResponseBuilder::advance_invoices)
    /// - [`created_at`](PostV1BankImportTemplatesGetResponseBuilder::created_at)
    /// - [`updated_at`](PostV1BankImportTemplatesGetResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1BankImportTemplatesGetResponse, BuildError> {
        Ok(PostV1BankImportTemplatesGetResponse {
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
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
