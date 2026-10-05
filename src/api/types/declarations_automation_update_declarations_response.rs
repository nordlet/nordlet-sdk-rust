pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationUpdateDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<AutomationUpdateDeclarationsResponseRowsItem>,
}

impl AutomationUpdateDeclarationsResponse {
    pub fn builder() -> AutomationUpdateDeclarationsResponseBuilder {
        <AutomationUpdateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationUpdateDeclarationsResponseBuilder {
    rows: Option<Vec<AutomationUpdateDeclarationsResponseRowsItem>>,
}

impl AutomationUpdateDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<AutomationUpdateDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationUpdateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](AutomationUpdateDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<AutomationUpdateDeclarationsResponse, BuildError> {
        Ok(AutomationUpdateDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
