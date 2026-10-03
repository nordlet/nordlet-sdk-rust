pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAutomationListResponseRowsItem {
    #[serde(rename = "ruleKey")]
    #[serde(default)]
    pub rule_key: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub applies: bool,
    #[serde(default)]
    pub configured: bool,
    pub certificate: PostV1DeclarationsAutomationListResponseRowsItemCertificate,
}

impl PostV1DeclarationsAutomationListResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsAutomationListResponseRowsItemBuilder {
        <PostV1DeclarationsAutomationListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAutomationListResponseRowsItemBuilder {
    rule_key: Option<String>,
    title: Option<String>,
    country: Option<String>,
    system: Option<String>,
    enabled: Option<bool>,
    applies: Option<bool>,
    configured: Option<bool>,
    certificate: Option<PostV1DeclarationsAutomationListResponseRowsItemCertificate>,
}

impl PostV1DeclarationsAutomationListResponseRowsItemBuilder {
    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn applies(mut self, value: bool) -> Self {
        self.applies = Some(value);
        self
    }

    pub fn configured(mut self, value: bool) -> Self {
        self.configured = Some(value);
        self
    }

    pub fn certificate(
        mut self,
        value: PostV1DeclarationsAutomationListResponseRowsItemCertificate,
    ) -> Self {
        self.certificate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAutomationListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::rule_key)
    /// - [`title`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::title)
    /// - [`country`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::country)
    /// - [`system`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::system)
    /// - [`enabled`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::enabled)
    /// - [`applies`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::applies)
    /// - [`configured`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::configured)
    /// - [`certificate`](PostV1DeclarationsAutomationListResponseRowsItemBuilder::certificate)
    pub fn build(self) -> Result<PostV1DeclarationsAutomationListResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsAutomationListResponseRowsItem {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            applies: self
                .applies
                .ok_or_else(|| BuildError::missing_field("applies"))?,
            configured: self
                .configured
                .ok_or_else(|| BuildError::missing_field("configured"))?,
            certificate: self
                .certificate
                .ok_or_else(|| BuildError::missing_field("certificate"))?,
        })
    }
}
