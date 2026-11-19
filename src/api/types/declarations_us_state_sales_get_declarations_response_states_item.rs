pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsStateSalesGetDeclarationsResponseStatesItem {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub documents: f64,
    #[serde(default)]
    pub net: String,
}

impl UsStateSalesGetDeclarationsResponseStatesItem {
    pub fn builder() -> UsStateSalesGetDeclarationsResponseStatesItemBuilder {
        <UsStateSalesGetDeclarationsResponseStatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsStateSalesGetDeclarationsResponseStatesItemBuilder {
    state: Option<String>,
    documents: Option<f64>,
    net: Option<String>,
}

impl UsStateSalesGetDeclarationsResponseStatesItemBuilder {
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn documents(mut self, value: f64) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsStateSalesGetDeclarationsResponseStatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](UsStateSalesGetDeclarationsResponseStatesItemBuilder::state)
    /// - [`documents`](UsStateSalesGetDeclarationsResponseStatesItemBuilder::documents)
    /// - [`net`](UsStateSalesGetDeclarationsResponseStatesItemBuilder::net)
    pub fn build(self) -> Result<UsStateSalesGetDeclarationsResponseStatesItem, BuildError> {
        Ok(UsStateSalesGetDeclarationsResponseStatesItem {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
