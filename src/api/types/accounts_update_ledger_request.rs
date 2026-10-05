pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountsUpdateLedgerRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, Option<AccountsUpdateLedgerRequestTranslationsValue>>>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPostable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_postable: Option<bool>,
}

impl AccountsUpdateLedgerRequest {
    pub fn builder() -> AccountsUpdateLedgerRequestBuilder {
        <AccountsUpdateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsUpdateLedgerRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, Option<AccountsUpdateLedgerRequestTranslationsValue>>>,
    parent_id: Option<String>,
    is_postable: Option<bool>,
}

impl AccountsUpdateLedgerRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, Option<AccountsUpdateLedgerRequestTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
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

    /// Consumes the builder and constructs a [`AccountsUpdateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountsUpdateLedgerRequestBuilder::id)
    pub fn build(self) -> Result<AccountsUpdateLedgerRequest, BuildError> {
        Ok(AccountsUpdateLedgerRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            translations: self.translations,
            parent_id: self.parent_id,
            is_postable: self.is_postable,
        })
    }
}
