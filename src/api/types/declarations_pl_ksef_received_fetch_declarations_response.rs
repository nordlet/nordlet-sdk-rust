pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlKsefReceivedFetchDeclarationsResponse {
    #[serde(rename = "ksefNumber")]
    #[serde(default)]
    pub ksef_number: String,
    #[serde(default)]
    pub xml: String,
    #[serde(rename = "attachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
}

impl PlKsefReceivedFetchDeclarationsResponse {
    pub fn builder() -> PlKsefReceivedFetchDeclarationsResponseBuilder {
        <PlKsefReceivedFetchDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlKsefReceivedFetchDeclarationsResponseBuilder {
    ksef_number: Option<String>,
    xml: Option<String>,
    attached_to: Option<String>,
}

impl PlKsefReceivedFetchDeclarationsResponseBuilder {
    pub fn ksef_number(mut self, value: impl Into<String>) -> Self {
        self.ksef_number = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn attached_to(mut self, value: impl Into<String>) -> Self {
        self.attached_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlKsefReceivedFetchDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ksef_number`](PlKsefReceivedFetchDeclarationsResponseBuilder::ksef_number)
    /// - [`xml`](PlKsefReceivedFetchDeclarationsResponseBuilder::xml)
    pub fn build(self) -> Result<PlKsefReceivedFetchDeclarationsResponse, BuildError> {
        Ok(PlKsefReceivedFetchDeclarationsResponse {
            ksef_number: self
                .ksef_number
                .ok_or_else(|| BuildError::missing_field("ksef_number"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            attached_to: self.attached_to,
        })
    }
}
