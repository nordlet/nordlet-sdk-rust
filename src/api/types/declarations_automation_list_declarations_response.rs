pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationListDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<AutomationListDeclarationsResponseRowsItem>,
}

impl AutomationListDeclarationsResponse {
    pub fn builder() -> AutomationListDeclarationsResponseBuilder {
        <AutomationListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationListDeclarationsResponseBuilder {
    rows: Option<Vec<AutomationListDeclarationsResponseRowsItem>>,
}

impl AutomationListDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<AutomationListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](AutomationListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<AutomationListDeclarationsResponse, BuildError> {
        Ok(AutomationListDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
