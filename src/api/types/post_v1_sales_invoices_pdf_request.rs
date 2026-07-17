pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPdfRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1SalesInvoicesPdfRequestLocale>,
}

impl PostV1SalesInvoicesPdfRequest {
    pub fn builder() -> PostV1SalesInvoicesPdfRequestBuilder {
        <PostV1SalesInvoicesPdfRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPdfRequestBuilder {
    id: Option<String>,
    locale: Option<PostV1SalesInvoicesPdfRequestLocale>,
}

impl PostV1SalesInvoicesPdfRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn locale(mut self, value: PostV1SalesInvoicesPdfRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPdfRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesPdfRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesPdfRequest, BuildError> {
        Ok(PostV1SalesInvoicesPdfRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            locale: self.locale,
        })
    }
}
