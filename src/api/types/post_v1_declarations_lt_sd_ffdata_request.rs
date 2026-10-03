pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSdFfdataRequest {
    pub r#type: PostV1DeclarationsLtSdFfdataRequestType,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "managerFullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_full_name: Option<String>,
    #[serde(rename = "preparatorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparator_details: Option<String>,
}

impl PostV1DeclarationsLtSdFfdataRequest {
    pub fn builder() -> PostV1DeclarationsLtSdFfdataRequestBuilder {
        <PostV1DeclarationsLtSdFfdataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSdFfdataRequestBuilder {
    r#type: Option<PostV1DeclarationsLtSdFfdataRequestType>,
    from_date: Option<String>,
    to_date: Option<String>,
    manager_full_name: Option<String>,
    preparator_details: Option<String>,
}

impl PostV1DeclarationsLtSdFfdataRequestBuilder {
    pub fn r#type(mut self, value: PostV1DeclarationsLtSdFfdataRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn manager_full_name(mut self, value: impl Into<String>) -> Self {
        self.manager_full_name = Some(value.into());
        self
    }

    pub fn preparator_details(mut self, value: impl Into<String>) -> Self {
        self.preparator_details = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSdFfdataRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostV1DeclarationsLtSdFfdataRequestBuilder::r#type)
    /// - [`from_date`](PostV1DeclarationsLtSdFfdataRequestBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsLtSdFfdataRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1DeclarationsLtSdFfdataRequest, BuildError> {
        Ok(PostV1DeclarationsLtSdFfdataRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            manager_full_name: self.manager_full_name,
            preparator_details: self.preparator_details,
        })
    }
}
