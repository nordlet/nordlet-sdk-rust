pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "typeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "autoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostV1AgreementsAgreementsUpdateRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1AgreementsAgreementsUpdateRequest {
    pub fn builder() -> PostV1AgreementsAgreementsUpdateRequestBuilder {
        <PostV1AgreementsAgreementsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsUpdateRequestBuilder {
    id: Option<String>,
    type_id: Option<String>,
    name: Option<String>,
    end_date: Option<String>,
    auto_renew: Option<bool>,
    value: Option<String>,
    status: Option<PostV1AgreementsAgreementsUpdateRequestStatus>,
    notes: Option<String>,
}

impl PostV1AgreementsAgreementsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn type_id(mut self, value: impl Into<String>) -> Self {
        self.type_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1AgreementsAgreementsUpdateRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsUpdateRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            type_id: self.type_id,
            name: self.name,
            end_date: self.end_date,
            auto_renew: self.auto_renew,
            value: self.value,
            status: self.status,
            notes: self.notes,
        })
    }
}
