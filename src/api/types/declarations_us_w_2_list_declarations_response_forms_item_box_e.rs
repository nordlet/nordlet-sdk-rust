pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsW2ListDeclarationsResponseFormsItemBoxE {
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
}

impl UsW2ListDeclarationsResponseFormsItemBoxE {
    pub fn builder() -> UsW2ListDeclarationsResponseFormsItemBoxEBuilder {
        <UsW2ListDeclarationsResponseFormsItemBoxEBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsResponseFormsItemBoxEBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
}

impl UsW2ListDeclarationsResponseFormsItemBoxEBuilder {
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsResponseFormsItemBoxE`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_name`](UsW2ListDeclarationsResponseFormsItemBoxEBuilder::first_name)
    /// - [`last_name`](UsW2ListDeclarationsResponseFormsItemBoxEBuilder::last_name)
    pub fn build(self) -> Result<UsW2ListDeclarationsResponseFormsItemBoxE, BuildError> {
        Ok(UsW2ListDeclarationsResponseFormsItemBoxE {
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
        })
    }
}
