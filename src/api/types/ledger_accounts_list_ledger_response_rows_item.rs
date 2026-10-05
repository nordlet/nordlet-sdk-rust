pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountsListLedgerResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<AccountsListLedgerResponseRowsItemTranslationsValue>>>,
    pub r#type: AccountsListLedgerResponseRowsItemType,
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

impl AccountsListLedgerResponseRowsItem {
    pub fn builder() -> AccountsListLedgerResponseRowsItemBuilder {
        <AccountsListLedgerResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListLedgerResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    translations:
        Option<HashMap<String, Option<AccountsListLedgerResponseRowsItemTranslationsValue>>>,
    r#type: Option<AccountsListLedgerResponseRowsItemType>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl AccountsListLedgerResponseRowsItemBuilder {
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
        value: HashMap<String, Option<AccountsListLedgerResponseRowsItemTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    pub fn r#type(mut self, value: AccountsListLedgerResponseRowsItemType) -> Self {
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

    /// Consumes the builder and constructs a [`AccountsListLedgerResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountsListLedgerResponseRowsItemBuilder::id)
    /// - [`code`](AccountsListLedgerResponseRowsItemBuilder::code)
    /// - [`name`](AccountsListLedgerResponseRowsItemBuilder::name)
    /// - [`r#type`](AccountsListLedgerResponseRowsItemBuilder::r#type)
    /// - [`is_postable`](AccountsListLedgerResponseRowsItemBuilder::is_postable)
    /// - [`created_at`](AccountsListLedgerResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<AccountsListLedgerResponseRowsItem, BuildError> {
        Ok(AccountsListLedgerResponseRowsItem {
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
