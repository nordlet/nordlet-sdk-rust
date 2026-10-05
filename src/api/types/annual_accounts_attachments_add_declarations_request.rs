pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsAttachmentsAddDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    pub kind: AnnualAccountsAttachmentsAddDeclarationsRequestKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub r#ref: String,
}

impl AnnualAccountsAttachmentsAddDeclarationsRequest {
    pub fn builder() -> AnnualAccountsAttachmentsAddDeclarationsRequestBuilder {
        <AnnualAccountsAttachmentsAddDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsAttachmentsAddDeclarationsRequestBuilder {
    year: Option<i64>,
    kind: Option<AnnualAccountsAttachmentsAddDeclarationsRequestKind>,
    name: Option<String>,
    r#ref: Option<String>,
}

impl AnnualAccountsAttachmentsAddDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(mut self, value: AnnualAccountsAttachmentsAddDeclarationsRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsAttachmentsAddDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AnnualAccountsAttachmentsAddDeclarationsRequestBuilder::year)
    /// - [`kind`](AnnualAccountsAttachmentsAddDeclarationsRequestBuilder::kind)
    /// - [`r#ref`](AnnualAccountsAttachmentsAddDeclarationsRequestBuilder::r#ref)
    pub fn build(self) -> Result<AnnualAccountsAttachmentsAddDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsAttachmentsAddDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
