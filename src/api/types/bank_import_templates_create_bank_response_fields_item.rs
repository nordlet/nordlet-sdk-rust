pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesCreateBankResponseFieldsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "createPartner")]
    #[serde(default)]
    pub create_partner: bool,
}

impl ImportTemplatesCreateBankResponseFieldsItem {
    pub fn builder() -> ImportTemplatesCreateBankResponseFieldsItemBuilder {
        <ImportTemplatesCreateBankResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesCreateBankResponseFieldsItemBuilder {
    name: Option<String>,
    account_code: Option<String>,
    create_partner: Option<bool>,
}

impl ImportTemplatesCreateBankResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`ImportTemplatesCreateBankResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ImportTemplatesCreateBankResponseFieldsItemBuilder::name)
    /// - [`create_partner`](ImportTemplatesCreateBankResponseFieldsItemBuilder::create_partner)
    pub fn build(self) -> Result<ImportTemplatesCreateBankResponseFieldsItem, BuildError> {
        Ok(ImportTemplatesCreateBankResponseFieldsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
            create_partner: self
                .create_partner
                .ok_or_else(|| BuildError::missing_field("create_partner"))?,
        })
    }
}
