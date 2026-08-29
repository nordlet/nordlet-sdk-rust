pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsBanksListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl PostV1BankFeedsBanksListRequest {
    pub fn builder() -> PostV1BankFeedsBanksListRequestBuilder {
        <PostV1BankFeedsBanksListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsBanksListRequestBuilder {
    country: Option<String>,
}

impl PostV1BankFeedsBanksListRequestBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsBanksListRequest`].
    pub fn build(self) -> Result<PostV1BankFeedsBanksListRequest, BuildError> {
        Ok(PostV1BankFeedsBanksListRequest {
            country: self.country,
        })
    }
}
