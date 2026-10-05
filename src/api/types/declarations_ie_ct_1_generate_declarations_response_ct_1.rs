pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeCt1GenerateDeclarationsResponseCt1 {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
}

impl IeCt1GenerateDeclarationsResponseCt1 {
    pub fn builder() -> IeCt1GenerateDeclarationsResponseCt1Builder {
        <IeCt1GenerateDeclarationsResponseCt1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeCt1GenerateDeclarationsResponseCt1Builder {
    file_name: Option<String>,
    xml: Option<String>,
}

impl IeCt1GenerateDeclarationsResponseCt1Builder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IeCt1GenerateDeclarationsResponseCt1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](IeCt1GenerateDeclarationsResponseCt1Builder::file_name)
    /// - [`xml`](IeCt1GenerateDeclarationsResponseCt1Builder::xml)
    pub fn build(self) -> Result<IeCt1GenerateDeclarationsResponseCt1, BuildError> {
        Ok(IeCt1GenerateDeclarationsResponseCt1 {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
