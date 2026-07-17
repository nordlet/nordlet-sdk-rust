pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsCreateRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1SalesActsCreateRequestType>,
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
    pub lines: Option<Vec<PostV1SalesActsCreateRequestLinesItem>>,
}

impl PostV1SalesActsCreateRequest {
    pub fn builder() -> PostV1SalesActsCreateRequestBuilder {
        <PostV1SalesActsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsCreateRequestBuilder {
    partner_id: Option<String>,
    r#type: Option<PostV1SalesActsCreateRequestType>,
    document_date: Option<String>,
    sale_invoice_id: Option<String>,
    transferred_by_name: Option<String>,
    transferred_by_title: Option<String>,
    accepted_by_name: Option<String>,
    accepted_by_title: Option<String>,
    notes: Option<String>,
    series: Option<String>,
    lines: Option<Vec<PostV1SalesActsCreateRequestLinesItem>>,
}

impl PostV1SalesActsCreateRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1SalesActsCreateRequestType) -> Self {
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

    pub fn lines(mut self, value: Vec<PostV1SalesActsCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1SalesActsCreateRequestBuilder::partner_id)
    pub fn build(self) -> Result<PostV1SalesActsCreateRequest, BuildError> {
        Ok(PostV1SalesActsCreateRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
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
        })
    }
}
