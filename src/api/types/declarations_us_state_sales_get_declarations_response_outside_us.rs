pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsStateSalesGetDeclarationsResponseOutsideUs {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub documents: f64,
    #[serde(default)]
    pub net: String,
}

impl UsStateSalesGetDeclarationsResponseOutsideUs {
    pub fn builder() -> UsStateSalesGetDeclarationsResponseOutsideUsBuilder {
        <UsStateSalesGetDeclarationsResponseOutsideUsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsStateSalesGetDeclarationsResponseOutsideUsBuilder {
    documents: Option<f64>,
    net: Option<String>,
}

impl UsStateSalesGetDeclarationsResponseOutsideUsBuilder {
    pub fn documents(mut self, value: f64) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsStateSalesGetDeclarationsResponseOutsideUs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](UsStateSalesGetDeclarationsResponseOutsideUsBuilder::documents)
    /// - [`net`](UsStateSalesGetDeclarationsResponseOutsideUsBuilder::net)
    pub fn build(self) -> Result<UsStateSalesGetDeclarationsResponseOutsideUs, BuildError> {
        Ok(UsStateSalesGetDeclarationsResponseOutsideUs {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
