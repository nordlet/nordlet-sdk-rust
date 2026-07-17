pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub r#type: PostV1SalesActsGetResponseType,
    pub status: PostV1SalesActsGetResponseStatus,
    #[serde(default)]
    pub series: String,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: String,
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
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1SalesActsGetResponseLinesItem>,
}

impl PostV1SalesActsGetResponse {
    pub fn builder() -> PostV1SalesActsGetResponseBuilder {
        <PostV1SalesActsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsGetResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<PostV1SalesActsGetResponseType>,
    status: Option<PostV1SalesActsGetResponseStatus>,
    series: Option<String>,
    full_number: Option<String>,
    document_date: Option<String>,
    sale_invoice_id: Option<String>,
    transferred_by_name: Option<String>,
    transferred_by_title: Option<String>,
    accepted_by_name: Option<String>,
    accepted_by_title: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    lines: Option<Vec<PostV1SalesActsGetResponseLinesItem>>,
}

impl PostV1SalesActsGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1SalesActsGetResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1SalesActsGetResponseStatus) -> Self {
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

    pub fn document_date(mut self, value: impl Into<String>) -> Self {
        self.document_date = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1SalesActsGetResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsGetResponseBuilder::id)
    /// - [`partner_id`](PostV1SalesActsGetResponseBuilder::partner_id)
    /// - [`r#type`](PostV1SalesActsGetResponseBuilder::r#type)
    /// - [`status`](PostV1SalesActsGetResponseBuilder::status)
    /// - [`series`](PostV1SalesActsGetResponseBuilder::series)
    /// - [`document_date`](PostV1SalesActsGetResponseBuilder::document_date)
    /// - [`created_at`](PostV1SalesActsGetResponseBuilder::created_at)
    /// - [`updated_at`](PostV1SalesActsGetResponseBuilder::updated_at)
    /// - [`lines`](PostV1SalesActsGetResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1SalesActsGetResponse, BuildError> {
        Ok(PostV1SalesActsGetResponse {
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
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
