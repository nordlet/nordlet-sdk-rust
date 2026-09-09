pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsConvertRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_type: Option<PostV1LeadsConvertRequestPartnerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
}

impl PostV1LeadsConvertRequest {
    pub fn builder() -> PostV1LeadsConvertRequestBuilder {
        <PostV1LeadsConvertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsConvertRequestBuilder {
    id: Option<String>,
    partner_type: Option<PostV1LeadsConvertRequestPartnerType>,
    code: Option<String>,
    vat_code: Option<String>,
}

impl PostV1LeadsConvertRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_type(mut self, value: PostV1LeadsConvertRequestPartnerType) -> Self {
        self.partner_type = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsConvertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsConvertRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsConvertRequest, BuildError> {
        Ok(PostV1LeadsConvertRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_type: self.partner_type,
            code: self.code,
            vat_code: self.vat_code,
        })
    }
}
