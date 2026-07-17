pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PartnersStatusesListResponseRowsItem {
    pub fn builder() -> PostV1PartnersStatusesListResponseRowsItemBuilder {
        <PostV1PartnersStatusesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    sort_order: Option<i64>,
    created_at: Option<String>,
}

impl PostV1PartnersStatusesListResponseRowsItemBuilder {
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

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersStatusesListResponseRowsItemBuilder::id)
    /// - [`code`](PostV1PartnersStatusesListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1PartnersStatusesListResponseRowsItemBuilder::name)
    /// - [`sort_order`](PostV1PartnersStatusesListResponseRowsItemBuilder::sort_order)
    /// - [`created_at`](PostV1PartnersStatusesListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1PartnersStatusesListResponseRowsItem, BuildError> {
        Ok(PostV1PartnersStatusesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
