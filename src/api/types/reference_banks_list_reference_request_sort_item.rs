pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BanksListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<BanksListReferenceRequestSortItemDir>,
}

impl BanksListReferenceRequestSortItem {
    pub fn builder() -> BanksListReferenceRequestSortItemBuilder {
        <BanksListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BanksListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<BanksListReferenceRequestSortItemDir>,
}

impl BanksListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: BanksListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BanksListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BanksListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<BanksListReferenceRequestSortItem, BuildError> {
        Ok(BanksListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
