pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesRecordsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1HrEmployeesRecordsUpdateRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(rename = "validUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1HrEmployeesRecordsUpdateRequest {
    pub fn builder() -> PostV1HrEmployeesRecordsUpdateRequestBuilder {
        <PostV1HrEmployeesRecordsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsUpdateRequestBuilder {
    id: Option<String>,
    r#type: Option<PostV1HrEmployeesRecordsUpdateRequestType>,
    title: Option<String>,
    institution: Option<String>,
    issued_at: Option<String>,
    valid_until: Option<String>,
    file_id: Option<String>,
    notes: Option<String>,
}

impl PostV1HrEmployeesRecordsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1HrEmployeesRecordsUpdateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn institution(mut self, value: impl Into<String>) -> Self {
        self.institution = Some(value.into());
        self
    }

    pub fn issued_at(mut self, value: impl Into<String>) -> Self {
        self.issued_at = Some(value.into());
        self
    }

    pub fn valid_until(mut self, value: impl Into<String>) -> Self {
        self.valid_until = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesRecordsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsUpdateRequest, BuildError> {
        Ok(PostV1HrEmployeesRecordsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type,
            title: self.title,
            institution: self.institution,
            issued_at: self.issued_at,
            valid_until: self.valid_until,
            file_id: self.file_id,
            notes: self.notes,
        })
    }
}
