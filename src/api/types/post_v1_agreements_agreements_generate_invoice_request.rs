pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsGenerateInvoiceRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<String>,
}

impl PostV1AgreementsAgreementsGenerateInvoiceRequest {
    pub fn builder() -> PostV1AgreementsAgreementsGenerateInvoiceRequestBuilder {
        <PostV1AgreementsAgreementsGenerateInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsGenerateInvoiceRequestBuilder {
    id: Option<String>,
    as_of_date: Option<String>,
}

impl PostV1AgreementsAgreementsGenerateInvoiceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn as_of_date(mut self, value: impl Into<String>) -> Self {
        self.as_of_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsGenerateInvoiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsGenerateInvoiceRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsGenerateInvoiceRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsGenerateInvoiceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            as_of_date: self.as_of_date,
        })
    }
}
