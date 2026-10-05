pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProfileUpdateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ProfileUpdateAccountRequest {
    pub fn builder() -> ProfileUpdateAccountRequestBuilder {
        <ProfileUpdateAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProfileUpdateAccountRequestBuilder {
    name: Option<String>,
}

impl ProfileUpdateAccountRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProfileUpdateAccountRequest`].
    pub fn build(self) -> Result<ProfileUpdateAccountRequest, BuildError> {
        Ok(ProfileUpdateAccountRequest { name: self.name })
    }
}
