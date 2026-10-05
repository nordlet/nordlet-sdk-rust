pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtAnnualReturnGenerateDeclarationsResponseMembersItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default)]
    pub shares: String,
    #[serde(rename = "nominalValue")]
    #[serde(default)]
    pub nominal_value: String,
    #[serde(rename = "shareClass")]
    #[serde(default)]
    pub share_class: String,
}

impl MtAnnualReturnGenerateDeclarationsResponseMembersItem {
    pub fn builder() -> MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder {
        <MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder {
    name: Option<String>,
    identifier: Option<String>,
    shares: Option<String>,
    nominal_value: Option<String>,
    share_class: Option<String>,
}

impl MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn shares(mut self, value: impl Into<String>) -> Self {
        self.shares = Some(value.into());
        self
    }

    pub fn nominal_value(mut self, value: impl Into<String>) -> Self {
        self.nominal_value = Some(value.into());
        self
    }

    pub fn share_class(mut self, value: impl Into<String>) -> Self {
        self.share_class = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MtAnnualReturnGenerateDeclarationsResponseMembersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder::name)
    /// - [`shares`](MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder::shares)
    /// - [`nominal_value`](MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder::nominal_value)
    /// - [`share_class`](MtAnnualReturnGenerateDeclarationsResponseMembersItemBuilder::share_class)
    pub fn build(
        self,
    ) -> Result<MtAnnualReturnGenerateDeclarationsResponseMembersItem, BuildError> {
        Ok(MtAnnualReturnGenerateDeclarationsResponseMembersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
            shares: self
                .shares
                .ok_or_else(|| BuildError::missing_field("shares"))?,
            nominal_value: self
                .nominal_value
                .ok_or_else(|| BuildError::missing_field("nominal_value"))?,
            share_class: self
                .share_class
                .ok_or_else(|| BuildError::missing_field("share_class"))?,
        })
    }
}
