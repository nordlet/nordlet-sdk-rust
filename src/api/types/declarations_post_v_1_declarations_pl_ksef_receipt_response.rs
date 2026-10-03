pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlKsefReceiptResponse {
    #[serde(rename = "referenceNumber")]
    #[serde(default)]
    pub reference_number: String,
    pub state: PostV1DeclarationsPlKsefReceiptResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "invoiceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_count: Option<i64>,
    #[serde(rename = "upoXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upo_xml: Option<String>,
}

impl PostV1DeclarationsPlKsefReceiptResponse {
    pub fn builder() -> PostV1DeclarationsPlKsefReceiptResponseBuilder {
        <PostV1DeclarationsPlKsefReceiptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlKsefReceiptResponseBuilder {
    reference_number: Option<String>,
    state: Option<PostV1DeclarationsPlKsefReceiptResponseState>,
    detail: Option<String>,
    invoice_count: Option<i64>,
    upo_xml: Option<String>,
}

impl PostV1DeclarationsPlKsefReceiptResponseBuilder {
    pub fn reference_number(mut self, value: impl Into<String>) -> Self {
        self.reference_number = Some(value.into());
        self
    }

    pub fn state(mut self, value: PostV1DeclarationsPlKsefReceiptResponseState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn invoice_count(mut self, value: i64) -> Self {
        self.invoice_count = Some(value);
        self
    }

    pub fn upo_xml(mut self, value: impl Into<String>) -> Self {
        self.upo_xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlKsefReceiptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference_number`](PostV1DeclarationsPlKsefReceiptResponseBuilder::reference_number)
    /// - [`state`](PostV1DeclarationsPlKsefReceiptResponseBuilder::state)
    pub fn build(self) -> Result<PostV1DeclarationsPlKsefReceiptResponse, BuildError> {
        Ok(PostV1DeclarationsPlKsefReceiptResponse {
            reference_number: self
                .reference_number
                .ok_or_else(|| BuildError::missing_field("reference_number"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            detail: self.detail,
            invoice_count: self.invoice_count,
            upo_xml: self.upo_xml,
        })
    }
}
