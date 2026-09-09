pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesUnlockResponseVatEvidencePartner {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "vatValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_valid: Option<bool>,
    #[serde(rename = "vatValidatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_validated_at: Option<String>,
}

impl PostV1SalesInvoicesUnlockResponseVatEvidencePartner {
    pub fn builder() -> PostV1SalesInvoicesUnlockResponseVatEvidencePartnerBuilder {
        <PostV1SalesInvoicesUnlockResponseVatEvidencePartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUnlockResponseVatEvidencePartnerBuilder {
    id: Option<String>,
    vat_code: Option<String>,
    vat_valid: Option<bool>,
    vat_validated_at: Option<String>,
}

impl PostV1SalesInvoicesUnlockResponseVatEvidencePartnerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn vat_valid(mut self, value: bool) -> Self {
        self.vat_valid = Some(value);
        self
    }

    pub fn vat_validated_at(mut self, value: impl Into<String>) -> Self {
        self.vat_validated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUnlockResponseVatEvidencePartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesUnlockResponseVatEvidencePartnerBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesUnlockResponseVatEvidencePartner, BuildError> {
        Ok(PostV1SalesInvoicesUnlockResponseVatEvidencePartner {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            vat_code: self.vat_code,
            vat_valid: self.vat_valid,
            vat_validated_at: self.vat_validated_at,
        })
    }
}
