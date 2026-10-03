pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsAttachmentsAddRequest {
    #[serde(default)]
    pub year: i64,
    pub kind: PostV1DeclarationsAnnualAccountsAttachmentsAddRequestKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1DeclarationsAnnualAccountsAttachmentsAddRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder {
        <PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder {
    year: Option<i64>,
    kind: Option<PostV1DeclarationsAnnualAccountsAttachmentsAddRequestKind>,
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsAnnualAccountsAttachmentsAddRequestKind,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsAttachmentsAddRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder::year)
    /// - [`kind`](PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder::kind)
    /// - [`r#ref`](PostV1DeclarationsAnnualAccountsAttachmentsAddRequestBuilder::r#ref)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsAttachmentsAddRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsAttachmentsAddRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
