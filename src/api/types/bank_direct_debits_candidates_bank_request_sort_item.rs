pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DirectDebitsCandidatesBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<DirectDebitsCandidatesBankRequestSortItemDir>,
}

impl DirectDebitsCandidatesBankRequestSortItem {
    pub fn builder() -> DirectDebitsCandidatesBankRequestSortItemBuilder {
        <DirectDebitsCandidatesBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DirectDebitsCandidatesBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<DirectDebitsCandidatesBankRequestSortItemDir>,
}

impl DirectDebitsCandidatesBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: DirectDebitsCandidatesBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DirectDebitsCandidatesBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DirectDebitsCandidatesBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<DirectDebitsCandidatesBankRequestSortItem, BuildError> {
        Ok(DirectDebitsCandidatesBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
