pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesSendRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1SalesInvoicesSendRequestLocale>,
}

impl PostV1SalesInvoicesSendRequest {
    pub fn builder() -> PostV1SalesInvoicesSendRequestBuilder {
        <PostV1SalesInvoicesSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesSendRequestBuilder {
    id: Option<String>,
    to: Option<String>,
    locale: Option<PostV1SalesInvoicesSendRequestLocale>,
}

impl PostV1SalesInvoicesSendRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn locale(mut self, value: PostV1SalesInvoicesSendRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesSendRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesSendRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesSendRequest, BuildError> {
        Ok(PostV1SalesInvoicesSendRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            to: self.to,
            locale: self.locale,
        })
    }
}
