pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsListPosRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RefundsListPosRequestSortItemDir>,
}

impl RefundsListPosRequestSortItem {
    pub fn builder() -> RefundsListPosRequestSortItemBuilder {
        <RefundsListPosRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsListPosRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RefundsListPosRequestSortItemDir>,
}

impl RefundsListPosRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RefundsListPosRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundsListPosRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RefundsListPosRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RefundsListPosRequestSortItem, BuildError> {
        Ok(RefundsListPosRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
