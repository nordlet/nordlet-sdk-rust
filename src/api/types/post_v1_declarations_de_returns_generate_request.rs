pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnsGenerateRequest {
    #[serde(rename = "ruleKey")]
    pub rule_key: PostV1DeclarationsDeReturnsGenerateRequestRuleKey,
    #[serde(default)]
    pub period: String,
}

impl PostV1DeclarationsDeReturnsGenerateRequest {
    pub fn builder() -> PostV1DeclarationsDeReturnsGenerateRequestBuilder {
        <PostV1DeclarationsDeReturnsGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnsGenerateRequestBuilder {
    rule_key: Option<PostV1DeclarationsDeReturnsGenerateRequestRuleKey>,
    period: Option<String>,
}

impl PostV1DeclarationsDeReturnsGenerateRequestBuilder {
    pub fn rule_key(mut self, value: PostV1DeclarationsDeReturnsGenerateRequestRuleKey) -> Self {
        self.rule_key = Some(value);
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnsGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](PostV1DeclarationsDeReturnsGenerateRequestBuilder::rule_key)
    /// - [`period`](PostV1DeclarationsDeReturnsGenerateRequestBuilder::period)
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnsGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsDeReturnsGenerateRequest {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
