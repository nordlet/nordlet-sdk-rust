pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LeadsConvertResponse {
    pub lead: PostV1LeadsConvertResponseLead,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl PostV1LeadsConvertResponse {
    pub fn builder() -> PostV1LeadsConvertResponseBuilder {
        <PostV1LeadsConvertResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsConvertResponseBuilder {
    lead: Option<PostV1LeadsConvertResponseLead>,
    partner_id: Option<String>,
}

impl PostV1LeadsConvertResponseBuilder {
    pub fn lead(mut self, value: PostV1LeadsConvertResponseLead) -> Self {
        self.lead = Some(value);
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsConvertResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead`](PostV1LeadsConvertResponseBuilder::lead)
    /// - [`partner_id`](PostV1LeadsConvertResponseBuilder::partner_id)
    pub fn build(self) -> Result<PostV1LeadsConvertResponse, BuildError> {
        Ok(PostV1LeadsConvertResponse {
            lead: self.lead.ok_or_else(|| BuildError::missing_field("lead"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
