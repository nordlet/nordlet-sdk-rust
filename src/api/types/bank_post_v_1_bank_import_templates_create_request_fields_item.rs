pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesCreateRequestFieldsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "createPartner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_partner: Option<bool>,
}

impl PostV1BankImportTemplatesCreateRequestFieldsItem {
    pub fn builder() -> PostV1BankImportTemplatesCreateRequestFieldsItemBuilder {
        <PostV1BankImportTemplatesCreateRequestFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesCreateRequestFieldsItemBuilder {
    name: Option<String>,
    account_code: Option<String>,
    create_partner: Option<bool>,
}

impl PostV1BankImportTemplatesCreateRequestFieldsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn create_partner(mut self, value: bool) -> Self {
        self.create_partner = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesCreateRequestFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1BankImportTemplatesCreateRequestFieldsItemBuilder::name)
    pub fn build(self) -> Result<PostV1BankImportTemplatesCreateRequestFieldsItem, BuildError> {
        Ok(PostV1BankImportTemplatesCreateRequestFieldsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
            create_partner: self.create_partner,
        })
    }
}
