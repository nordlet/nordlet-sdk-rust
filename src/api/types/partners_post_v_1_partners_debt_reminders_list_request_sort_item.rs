pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDebtRemindersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersDebtRemindersListRequestSortItemDir>,
}

impl PostV1PartnersDebtRemindersListRequestSortItem {
    pub fn builder() -> PostV1PartnersDebtRemindersListRequestSortItemBuilder {
        <PostV1PartnersDebtRemindersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDebtRemindersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersDebtRemindersListRequestSortItemDir>,
}

impl PostV1PartnersDebtRemindersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersDebtRemindersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersDebtRemindersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersDebtRemindersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersDebtRemindersListRequestSortItem, BuildError> {
        Ok(PostV1PartnersDebtRemindersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
