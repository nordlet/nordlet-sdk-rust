pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InquiriesListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<InquiriesListPartnersRequestSortItemDir>,
}

impl InquiriesListPartnersRequestSortItem {
    pub fn builder() -> InquiriesListPartnersRequestSortItemBuilder {
        <InquiriesListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InquiriesListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<InquiriesListPartnersRequestSortItemDir>,
}

impl InquiriesListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: InquiriesListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InquiriesListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InquiriesListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<InquiriesListPartnersRequestSortItem, BuildError> {
        Ok(InquiriesListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
