pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsStateSalesGetDeclarationsResponseWithoutState {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub documents: f64,
    #[serde(default)]
    pub net: String,
}

impl UsStateSalesGetDeclarationsResponseWithoutState {
    pub fn builder() -> UsStateSalesGetDeclarationsResponseWithoutStateBuilder {
        <UsStateSalesGetDeclarationsResponseWithoutStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsStateSalesGetDeclarationsResponseWithoutStateBuilder {
    documents: Option<f64>,
    net: Option<String>,
}

impl UsStateSalesGetDeclarationsResponseWithoutStateBuilder {
    pub fn documents(mut self, value: f64) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsStateSalesGetDeclarationsResponseWithoutState`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](UsStateSalesGetDeclarationsResponseWithoutStateBuilder::documents)
    /// - [`net`](UsStateSalesGetDeclarationsResponseWithoutStateBuilder::net)
    pub fn build(self) -> Result<UsStateSalesGetDeclarationsResponseWithoutState, BuildError> {
        Ok(UsStateSalesGetDeclarationsResponseWithoutState {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
