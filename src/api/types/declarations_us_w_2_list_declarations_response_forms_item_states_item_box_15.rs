pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsW2ListDeclarationsResponseFormsItemStatesItemBox15 {
    #[serde(default)]
    pub state: String,
    #[serde(rename = "employerStateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_state_id: Option<String>,
}

impl UsW2ListDeclarationsResponseFormsItemStatesItemBox15 {
    pub fn builder() -> UsW2ListDeclarationsResponseFormsItemStatesItemBox15Builder {
        <UsW2ListDeclarationsResponseFormsItemStatesItemBox15Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsResponseFormsItemStatesItemBox15Builder {
    state: Option<String>,
    employer_state_id: Option<String>,
}

impl UsW2ListDeclarationsResponseFormsItemStatesItemBox15Builder {
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn employer_state_id(mut self, value: impl Into<String>) -> Self {
        self.employer_state_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsResponseFormsItemStatesItemBox15`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](UsW2ListDeclarationsResponseFormsItemStatesItemBox15Builder::state)
    pub fn build(self) -> Result<UsW2ListDeclarationsResponseFormsItemStatesItemBox15, BuildError> {
        Ok(UsW2ListDeclarationsResponseFormsItemStatesItemBox15 {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            employer_state_id: self.employer_state_id,
        })
    }
}
