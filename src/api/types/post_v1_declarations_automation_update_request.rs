pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAutomationUpdateRequest {
    #[serde(rename = "ruleKey")]
    #[serde(default)]
    pub rule_key: String,
    #[serde(default)]
    pub enabled: bool,
}

impl PostV1DeclarationsAutomationUpdateRequest {
    pub fn builder() -> PostV1DeclarationsAutomationUpdateRequestBuilder {
        <PostV1DeclarationsAutomationUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAutomationUpdateRequestBuilder {
    rule_key: Option<String>,
    enabled: Option<bool>,
}

impl PostV1DeclarationsAutomationUpdateRequestBuilder {
    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAutomationUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](PostV1DeclarationsAutomationUpdateRequestBuilder::rule_key)
    /// - [`enabled`](PostV1DeclarationsAutomationUpdateRequestBuilder::enabled)
    pub fn build(self) -> Result<PostV1DeclarationsAutomationUpdateRequest, BuildError> {
        Ok(PostV1DeclarationsAutomationUpdateRequest {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
