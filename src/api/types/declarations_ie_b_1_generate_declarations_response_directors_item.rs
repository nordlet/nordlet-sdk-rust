pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeB1GenerateDeclarationsResponseDirectorsItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "appointedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointed_on: Option<String>,
}

impl IeB1GenerateDeclarationsResponseDirectorsItem {
    pub fn builder() -> IeB1GenerateDeclarationsResponseDirectorsItemBuilder {
        <IeB1GenerateDeclarationsResponseDirectorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeB1GenerateDeclarationsResponseDirectorsItemBuilder {
    name: Option<String>,
    identifier: Option<String>,
    appointed_on: Option<String>,
}

impl IeB1GenerateDeclarationsResponseDirectorsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn appointed_on(mut self, value: impl Into<String>) -> Self {
        self.appointed_on = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IeB1GenerateDeclarationsResponseDirectorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](IeB1GenerateDeclarationsResponseDirectorsItemBuilder::name)
    pub fn build(self) -> Result<IeB1GenerateDeclarationsResponseDirectorsItem, BuildError> {
        Ok(IeB1GenerateDeclarationsResponseDirectorsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
            appointed_on: self.appointed_on,
        })
    }
}
