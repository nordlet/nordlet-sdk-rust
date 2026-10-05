pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazCancelDeclarationsRequest {
    #[serde(default)]
    pub entries: Vec<LtIvazCancelDeclarationsRequestEntriesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl LtIvazCancelDeclarationsRequest {
    pub fn builder() -> LtIvazCancelDeclarationsRequestBuilder {
        <LtIvazCancelDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazCancelDeclarationsRequestBuilder {
    entries: Option<Vec<LtIvazCancelDeclarationsRequestEntriesItem>>,
    persist: Option<bool>,
}

impl LtIvazCancelDeclarationsRequestBuilder {
    pub fn entries(mut self, value: Vec<LtIvazCancelDeclarationsRequestEntriesItem>) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIvazCancelDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](LtIvazCancelDeclarationsRequestBuilder::entries)
    pub fn build(self) -> Result<LtIvazCancelDeclarationsRequest, BuildError> {
        Ok(LtIvazCancelDeclarationsRequest {
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
            persist: self.persist,
        })
    }
}
