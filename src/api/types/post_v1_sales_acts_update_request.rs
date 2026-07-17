pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsUpdateRequest {
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1SalesActsUpdateRequestType>,
    #[serde(rename = "documentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1SalesActsUpdateRequestLinesItem>>,
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesActsUpdateRequest {
    pub fn builder() -> PostV1SalesActsUpdateRequestBuilder {
        <PostV1SalesActsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsUpdateRequestBuilder {
    partner_id: Option<String>,
    r#type: Option<PostV1SalesActsUpdateRequestType>,
    document_date: Option<String>,
    sale_invoice_id: Option<String>,
    transferred_by_name: Option<String>,
    transferred_by_title: Option<String>,
    accepted_by_name: Option<String>,
    accepted_by_title: Option<String>,
    notes: Option<String>,
    series: Option<String>,
    lines: Option<Vec<PostV1SalesActsUpdateRequestLinesItem>>,
    id: Option<String>,
}

impl PostV1SalesActsUpdateRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1SalesActsUpdateRequestType) -> Self {
        self.r#type = Some(value);
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

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1SalesActsUpdateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesActsUpdateRequest, BuildError> {
        Ok(PostV1SalesActsUpdateRequest {
            partner_id: self.partner_id,
            r#type: self.r#type,
            document_date: self.document_date,
            sale_invoice_id: self.sale_invoice_id,
            transferred_by_name: self.transferred_by_name,
            transferred_by_title: self.transferred_by_title,
            accepted_by_name: self.accepted_by_name,
            accepted_by_title: self.accepted_by_title,
            notes: self.notes,
            series: self.series,
            lines: self.lines,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
