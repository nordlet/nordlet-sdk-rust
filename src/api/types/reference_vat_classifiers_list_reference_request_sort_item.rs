pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatClassifiersListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<VatClassifiersListReferenceRequestSortItemDir>,
}

impl VatClassifiersListReferenceRequestSortItem {
    pub fn builder() -> VatClassifiersListReferenceRequestSortItemBuilder {
        <VatClassifiersListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<VatClassifiersListReferenceRequestSortItemDir>,
}

impl VatClassifiersListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: VatClassifiersListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatClassifiersListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](VatClassifiersListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<VatClassifiersListReferenceRequestSortItem, BuildError> {
        Ok(VatClassifiersListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
