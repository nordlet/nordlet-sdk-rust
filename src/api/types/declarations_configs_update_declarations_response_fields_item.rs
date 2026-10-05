pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigsUpdateDeclarationsResponseFieldsItem {
    #[serde(default)]
    pub key: String,
    pub kind: ConfigsUpdateDeclarationsResponseFieldsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl ConfigsUpdateDeclarationsResponseFieldsItem {
    pub fn builder() -> ConfigsUpdateDeclarationsResponseFieldsItemBuilder {
        <ConfigsUpdateDeclarationsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsUpdateDeclarationsResponseFieldsItemBuilder {
    key: Option<String>,
    kind: Option<ConfigsUpdateDeclarationsResponseFieldsItemKind>,
    multiline: Option<bool>,
    options: Option<Vec<String>>,
}

impl ConfigsUpdateDeclarationsResponseFieldsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ConfigsUpdateDeclarationsResponseFieldsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn multiline(mut self, value: bool) -> Self {
        self.multiline = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigsUpdateDeclarationsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ConfigsUpdateDeclarationsResponseFieldsItemBuilder::key)
    /// - [`kind`](ConfigsUpdateDeclarationsResponseFieldsItemBuilder::kind)
    pub fn build(self) -> Result<ConfigsUpdateDeclarationsResponseFieldsItem, BuildError> {
        Ok(ConfigsUpdateDeclarationsResponseFieldsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            multiline: self.multiline,
            options: self.options,
        })
    }
}
