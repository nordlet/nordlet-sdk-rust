pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsListDeclarationsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<SubmissionsListDeclarationsRequestSortItemDir>,
}

impl SubmissionsListDeclarationsRequestSortItem {
    pub fn builder() -> SubmissionsListDeclarationsRequestSortItemBuilder {
        <SubmissionsListDeclarationsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsListDeclarationsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<SubmissionsListDeclarationsRequestSortItemDir>,
}

impl SubmissionsListDeclarationsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: SubmissionsListDeclarationsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsListDeclarationsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SubmissionsListDeclarationsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<SubmissionsListDeclarationsRequestSortItem, BuildError> {
        Ok(SubmissionsListDeclarationsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
