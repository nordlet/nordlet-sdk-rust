pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigsListDeclarationsResponseRowsItemFieldsItem {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub kind: ConfigsListDeclarationsResponseRowsItemFieldsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    /// The allowed values of a `select` field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// The company profile field the value is taken from: `code`, `vatCode`, or `identifiers.<key>` for an entry of `identifiers`; change it with `account/companies/update`
    #[serde(rename = "companyField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_field: Option<String>,
}

impl ConfigsListDeclarationsResponseRowsItemFieldsItem {
    pub fn builder() -> ConfigsListDeclarationsResponseRowsItemFieldsItemBuilder {
        <ConfigsListDeclarationsResponseRowsItemFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsListDeclarationsResponseRowsItemFieldsItemBuilder {
    key: Option<String>,
    label: Option<String>,
    kind: Option<ConfigsListDeclarationsResponseRowsItemFieldsItemKind>,
    multiline: Option<bool>,
    options: Option<Vec<String>>,
    company_field: Option<String>,
}

impl ConfigsListDeclarationsResponseRowsItemFieldsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ConfigsListDeclarationsResponseRowsItemFieldsItemKind) -> Self {
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

    pub fn company_field(mut self, value: impl Into<String>) -> Self {
        self.company_field = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfigsListDeclarationsResponseRowsItemFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ConfigsListDeclarationsResponseRowsItemFieldsItemBuilder::key)
    /// - [`kind`](ConfigsListDeclarationsResponseRowsItemFieldsItemBuilder::kind)
    pub fn build(self) -> Result<ConfigsListDeclarationsResponseRowsItemFieldsItem, BuildError> {
        Ok(ConfigsListDeclarationsResponseRowsItemFieldsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            multiline: self.multiline,
            options: self.options,
            company_field: self.company_field,
        })
    }
}
