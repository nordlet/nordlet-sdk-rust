pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersCreateRequestPartner {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl PostV1EcommerceOrdersCreateRequestPartner {
    pub fn builder() -> PostV1EcommerceOrdersCreateRequestPartnerBuilder {
        <PostV1EcommerceOrdersCreateRequestPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersCreateRequestPartnerBuilder {
    name: Option<String>,
    email: Option<String>,
    code: Option<String>,
}

impl PostV1EcommerceOrdersCreateRequestPartnerBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersCreateRequestPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1EcommerceOrdersCreateRequestPartnerBuilder::name)
    pub fn build(self) -> Result<PostV1EcommerceOrdersCreateRequestPartner, BuildError> {
        Ok(PostV1EcommerceOrdersCreateRequestPartner {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            email: self.email,
            code: self.code,
        })
    }
}
