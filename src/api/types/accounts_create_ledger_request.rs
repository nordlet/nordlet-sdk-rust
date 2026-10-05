pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountsCreateLedgerRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, AccountsCreateLedgerRequestTranslationsValue>>,
    pub r#type: AccountsCreateLedgerRequestType,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_postable: Option<bool>,
}

impl AccountsCreateLedgerRequest {
    pub fn builder() -> AccountsCreateLedgerRequestBuilder {
        <AccountsCreateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsCreateLedgerRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, AccountsCreateLedgerRequestTranslationsValue>>,
    r#type: Option<AccountsCreateLedgerRequestType>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
}

impl AccountsCreateLedgerRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, AccountsCreateLedgerRequestTranslationsValue>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn r#type(mut self, value: AccountsCreateLedgerRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn is_postable(mut self, value: bool) -> Self {
        self.is_postable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsCreateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](AccountsCreateLedgerRequestBuilder::code)
    /// - [`name`](AccountsCreateLedgerRequestBuilder::name)
    /// - [`r#type`](AccountsCreateLedgerRequestBuilder::r#type)
    pub fn build(self) -> Result<AccountsCreateLedgerRequest, BuildError> {
        Ok(AccountsCreateLedgerRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            translations: self.translations,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            parent_id: self.parent_id,
            is_postable: self.is_postable,
        })
    }
}
