pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceBanksListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub bic: String,
    #[serde(rename = "bankCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
}

impl PostV1ReferenceBanksListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceBanksListResponseRowsItemBuilder {
        <PostV1ReferenceBanksListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceBanksListResponseRowsItemBuilder {
    id: Option<String>,
    country_code: Option<String>,
    name: Option<String>,
    bic: Option<String>,
    bank_code: Option<String>,
    is_active: Option<bool>,
}

impl PostV1ReferenceBanksListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceBanksListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ReferenceBanksListResponseRowsItemBuilder::id)
    /// - [`country_code`](PostV1ReferenceBanksListResponseRowsItemBuilder::country_code)
    /// - [`name`](PostV1ReferenceBanksListResponseRowsItemBuilder::name)
    /// - [`bic`](PostV1ReferenceBanksListResponseRowsItemBuilder::bic)
    /// - [`is_active`](PostV1ReferenceBanksListResponseRowsItemBuilder::is_active)
    pub fn build(self) -> Result<PostV1ReferenceBanksListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceBanksListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            bic: self.bic.ok_or_else(|| BuildError::missing_field("bic"))?,
            bank_code: self.bank_code,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
        })
    }
}
