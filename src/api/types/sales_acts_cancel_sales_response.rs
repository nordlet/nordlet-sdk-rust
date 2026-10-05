pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ActsCancelSalesResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub r#type: ActsCancelSalesResponseType,
    pub status: ActsCancelSalesResponseStatus,
    #[serde(default)]
    pub series: String,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: NaiveDate,
    #[serde(rename = "saleInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_invoice_id: Option<String>,
    #[serde(rename = "transferredByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_by_name: Option<String>,
    #[serde(rename = "transferredByTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_by_title: Option<String>,
    #[serde(rename = "acceptedByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_by_name: Option<String>,
    #[serde(rename = "acceptedByTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_by_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ActsCancelSalesResponse {
    pub fn builder() -> ActsCancelSalesResponseBuilder {
        <ActsCancelSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsCancelSalesResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<ActsCancelSalesResponseType>,
    status: Option<ActsCancelSalesResponseStatus>,
    series: Option<String>,
    full_number: Option<String>,
    document_date: Option<NaiveDate>,
    sale_invoice_id: Option<String>,
    transferred_by_name: Option<String>,
    transferred_by_title: Option<String>,
    accepted_by_name: Option<String>,
    accepted_by_title: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ActsCancelSalesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ActsCancelSalesResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: ActsCancelSalesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: NaiveDate) -> Self {
        self.document_date = Some(value);
        self
    }

    pub fn sale_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.sale_invoice_id = Some(value.into());
        self
    }

    pub fn transferred_by_name(mut self, value: impl Into<String>) -> Self {
        self.transferred_by_name = Some(value.into());
        self
    }

    pub fn transferred_by_title(mut self, value: impl Into<String>) -> Self {
        self.transferred_by_title = Some(value.into());
        self
    }

    pub fn accepted_by_name(mut self, value: impl Into<String>) -> Self {
        self.accepted_by_name = Some(value.into());
        self
    }

    pub fn accepted_by_title(mut self, value: impl Into<String>) -> Self {
        self.accepted_by_title = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
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

    /// Consumes the builder and constructs a [`ActsCancelSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ActsCancelSalesResponseBuilder::id)
    /// - [`partner_id`](ActsCancelSalesResponseBuilder::partner_id)
    /// - [`r#type`](ActsCancelSalesResponseBuilder::r#type)
    /// - [`status`](ActsCancelSalesResponseBuilder::status)
    /// - [`series`](ActsCancelSalesResponseBuilder::series)
    /// - [`document_date`](ActsCancelSalesResponseBuilder::document_date)
    /// - [`created_at`](ActsCancelSalesResponseBuilder::created_at)
    /// - [`updated_at`](ActsCancelSalesResponseBuilder::updated_at)
    pub fn build(self) -> Result<ActsCancelSalesResponse, BuildError> {
        Ok(ActsCancelSalesResponse {
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
            series: self
                .series
                .ok_or_else(|| BuildError::missing_field("series"))?,
            full_number: self.full_number,
            document_date: self
                .document_date
                .ok_or_else(|| BuildError::missing_field("document_date"))?,
            sale_invoice_id: self.sale_invoice_id,
            transferred_by_name: self.transferred_by_name,
            transferred_by_title: self.transferred_by_title,
            accepted_by_name: self.accepted_by_name,
            accepted_by_title: self.accepted_by_title,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
