pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CnCodesListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<CnCodesListReferenceRequestSortItemDir>,
}

impl CnCodesListReferenceRequestSortItem {
    pub fn builder() -> CnCodesListReferenceRequestSortItemBuilder {
        <CnCodesListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CnCodesListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<CnCodesListReferenceRequestSortItemDir>,
}

impl CnCodesListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: CnCodesListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CnCodesListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CnCodesListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<CnCodesListReferenceRequestSortItem, BuildError> {
        Ok(CnCodesListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
