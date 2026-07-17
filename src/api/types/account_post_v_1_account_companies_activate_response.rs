pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesActivateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl PostV1AccountCompaniesActivateResponse {
    pub fn builder() -> PostV1AccountCompaniesActivateResponseBuilder {
        <PostV1AccountCompaniesActivateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesActivateResponseBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl PostV1AccountCompaniesActivateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesActivateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountCompaniesActivateResponseBuilder::id)
    /// - [`status`](PostV1AccountCompaniesActivateResponseBuilder::status)
    pub fn build(self) -> Result<PostV1AccountCompaniesActivateResponse, BuildError> {
        Ok(PostV1AccountCompaniesActivateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
