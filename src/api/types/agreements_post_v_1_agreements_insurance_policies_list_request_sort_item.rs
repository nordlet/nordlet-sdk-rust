pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsInsurancePoliciesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1AgreementsInsurancePoliciesListRequestSortItemDir>,
}

impl PostV1AgreementsInsurancePoliciesListRequestSortItem {
    pub fn builder() -> PostV1AgreementsInsurancePoliciesListRequestSortItemBuilder {
        <PostV1AgreementsInsurancePoliciesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsInsurancePoliciesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1AgreementsInsurancePoliciesListRequestSortItemDir>,
}

impl PostV1AgreementsInsurancePoliciesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1AgreementsInsurancePoliciesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsInsurancePoliciesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AgreementsInsurancePoliciesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1AgreementsInsurancePoliciesListRequestSortItem, BuildError> {
        Ok(PostV1AgreementsInsurancePoliciesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
