pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeB1GenerateDeclarationsResponseSecretary {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl IeB1GenerateDeclarationsResponseSecretary {
    pub fn builder() -> IeB1GenerateDeclarationsResponseSecretaryBuilder {
        <IeB1GenerateDeclarationsResponseSecretaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeB1GenerateDeclarationsResponseSecretaryBuilder {
    name: Option<String>,
    identifier: Option<String>,
}

impl IeB1GenerateDeclarationsResponseSecretaryBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IeB1GenerateDeclarationsResponseSecretary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](IeB1GenerateDeclarationsResponseSecretaryBuilder::name)
    pub fn build(self) -> Result<IeB1GenerateDeclarationsResponseSecretary, BuildError> {
        Ok(IeB1GenerateDeclarationsResponseSecretary {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
        })
    }
}
