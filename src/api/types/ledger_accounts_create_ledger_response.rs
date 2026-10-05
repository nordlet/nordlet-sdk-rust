pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountsCreateLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<AccountsCreateLedgerResponseTranslationsValue>>>,
    pub r#type: AccountsCreateLedgerResponseType,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(default)]
    pub is_postable: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl AccountsCreateLedgerResponse {
    pub fn builder() -> AccountsCreateLedgerResponseBuilder {
        <AccountsCreateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsCreateLedgerResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, Option<AccountsCreateLedgerResponseTranslationsValue>>>,
    r#type: Option<AccountsCreateLedgerResponseType>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl AccountsCreateLedgerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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
        value: HashMap<String, Option<AccountsCreateLedgerResponseTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn r#type(mut self, value: AccountsCreateLedgerResponseType) -> Self {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsCreateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountsCreateLedgerResponseBuilder::id)
    /// - [`code`](AccountsCreateLedgerResponseBuilder::code)
    /// - [`name`](AccountsCreateLedgerResponseBuilder::name)
    /// - [`r#type`](AccountsCreateLedgerResponseBuilder::r#type)
    /// - [`is_postable`](AccountsCreateLedgerResponseBuilder::is_postable)
    /// - [`created_at`](AccountsCreateLedgerResponseBuilder::created_at)
    pub fn build(self) -> Result<AccountsCreateLedgerResponse, BuildError> {
        Ok(AccountsCreateLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            translations: self.translations,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            parent_id: self.parent_id,
            is_postable: self
                .is_postable
                .ok_or_else(|| BuildError::missing_field("is_postable"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
