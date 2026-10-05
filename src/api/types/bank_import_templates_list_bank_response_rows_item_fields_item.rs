pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesListBankResponseRowsItemFieldsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "createPartner")]
    #[serde(default)]
    pub create_partner: bool,
}

impl ImportTemplatesListBankResponseRowsItemFieldsItem {
    pub fn builder() -> ImportTemplatesListBankResponseRowsItemFieldsItemBuilder {
        <ImportTemplatesListBankResponseRowsItemFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesListBankResponseRowsItemFieldsItemBuilder {
    name: Option<String>,
    account_code: Option<String>,
    create_partner: Option<bool>,
}

impl ImportTemplatesListBankResponseRowsItemFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`ImportTemplatesListBankResponseRowsItemFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ImportTemplatesListBankResponseRowsItemFieldsItemBuilder::name)
    /// - [`create_partner`](ImportTemplatesListBankResponseRowsItemFieldsItemBuilder::create_partner)
    pub fn build(self) -> Result<ImportTemplatesListBankResponseRowsItemFieldsItem, BuildError> {
        Ok(ImportTemplatesListBankResponseRowsItemFieldsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
            create_partner: self
                .create_partner
                .ok_or_else(|| BuildError::missing_field("create_partner"))?,
        })
    }
}
