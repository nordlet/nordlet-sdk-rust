pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesUpdateBankRequestFieldsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "createPartner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_partner: Option<bool>,
}

impl ImportTemplatesUpdateBankRequestFieldsItem {
    pub fn builder() -> ImportTemplatesUpdateBankRequestFieldsItemBuilder {
        <ImportTemplatesUpdateBankRequestFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesUpdateBankRequestFieldsItemBuilder {
    name: Option<String>,
    account_code: Option<String>,
    create_partner: Option<bool>,
}

impl ImportTemplatesUpdateBankRequestFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`ImportTemplatesUpdateBankRequestFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ImportTemplatesUpdateBankRequestFieldsItemBuilder::name)
    pub fn build(self) -> Result<ImportTemplatesUpdateBankRequestFieldsItem, BuildError> {
        Ok(ImportTemplatesUpdateBankRequestFieldsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
            create_partner: self.create_partner,
        })
    }
}
