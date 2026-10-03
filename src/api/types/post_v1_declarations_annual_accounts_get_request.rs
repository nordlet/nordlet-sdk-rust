pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsGetRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsAnnualAccountsGetRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsGetRequestBuilder {
        <PostV1DeclarationsAnnualAccountsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsGetRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsAnnualAccountsGetRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsAnnualAccountsGetRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsAnnualAccountsGetRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsGetRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
