pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesCreateBankRequestFieldsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "createPartner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_partner: Option<bool>,
}

impl ImportTemplatesCreateBankRequestFieldsItem {
    pub fn builder() -> ImportTemplatesCreateBankRequestFieldsItemBuilder {
        <ImportTemplatesCreateBankRequestFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesCreateBankRequestFieldsItemBuilder {
    name: Option<String>,
    account_code: Option<String>,
    create_partner: Option<bool>,
}

impl ImportTemplatesCreateBankRequestFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`ImportTemplatesCreateBankRequestFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ImportTemplatesCreateBankRequestFieldsItemBuilder::name)
    pub fn build(self) -> Result<ImportTemplatesCreateBankRequestFieldsItem, BuildError> {
        Ok(ImportTemplatesCreateBankRequestFieldsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
            create_partner: self.create_partner,
        })
    }
}
