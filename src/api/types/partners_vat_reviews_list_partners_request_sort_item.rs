pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatReviewsListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<VatReviewsListPartnersRequestSortItemDir>,
}

impl VatReviewsListPartnersRequestSortItem {
    pub fn builder() -> VatReviewsListPartnersRequestSortItemBuilder {
        <VatReviewsListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatReviewsListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<VatReviewsListPartnersRequestSortItemDir>,
}

impl VatReviewsListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: VatReviewsListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatReviewsListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](VatReviewsListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<VatReviewsListPartnersRequestSortItem, BuildError> {
        Ok(VatReviewsListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
