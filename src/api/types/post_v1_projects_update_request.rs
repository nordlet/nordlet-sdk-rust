pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostV1ProjectsUpdateRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProjectsUpdateRequest {
    pub fn builder() -> PostV1ProjectsUpdateRequestBuilder {
        <PostV1ProjectsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsUpdateRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    partner_id: Option<String>,
    status: Option<PostV1ProjectsUpdateRequestStatus>,
    notes: Option<String>,
}

impl PostV1ProjectsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1ProjectsUpdateRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProjectsUpdateRequest, BuildError> {
        Ok(PostV1ProjectsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            partner_id: self.partner_id,
            status: self.status,
            notes: self.notes,
        })
    }
}
