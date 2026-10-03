pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeB1GenerateResponseMembersItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "sharesQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_quantity: Option<String>,
    #[serde(rename = "sharesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_amount: Option<String>,
    #[serde(rename = "sharesType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_type: Option<String>,
    #[serde(rename = "acquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseMembersItem {
    pub fn builder() -> PostV1DeclarationsIeB1GenerateResponseMembersItemBuilder {
        <PostV1DeclarationsIeB1GenerateResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeB1GenerateResponseMembersItemBuilder {
    name: Option<String>,
    identifier: Option<String>,
    shares_quantity: Option<String>,
    shares_amount: Option<String>,
    shares_type: Option<String>,
    acquisition_date: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseMembersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn shares_quantity(mut self, value: impl Into<String>) -> Self {
        self.shares_quantity = Some(value.into());
        self
    }

    pub fn shares_amount(mut self, value: impl Into<String>) -> Self {
        self.shares_amount = Some(value.into());
        self
    }

    pub fn shares_type(mut self, value: impl Into<String>) -> Self {
        self.shares_type = Some(value.into());
        self
    }

    pub fn acquisition_date(mut self, value: impl Into<String>) -> Self {
        self.acquisition_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeB1GenerateResponseMembersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1DeclarationsIeB1GenerateResponseMembersItemBuilder::name)
    pub fn build(self) -> Result<PostV1DeclarationsIeB1GenerateResponseMembersItem, BuildError> {
        Ok(PostV1DeclarationsIeB1GenerateResponseMembersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
            shares_quantity: self.shares_quantity,
            shares_amount: self.shares_amount,
            shares_type: self.shares_type,
            acquisition_date: self.acquisition_date,
        })
    }
}
