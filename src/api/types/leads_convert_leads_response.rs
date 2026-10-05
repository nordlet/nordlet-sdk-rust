pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConvertLeadsResponse {
    pub lead: ConvertLeadsResponseLead,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl ConvertLeadsResponse {
    pub fn builder() -> ConvertLeadsResponseBuilder {
        <ConvertLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertLeadsResponseBuilder {
    lead: Option<ConvertLeadsResponseLead>,
    partner_id: Option<String>,
}

impl ConvertLeadsResponseBuilder {
    pub fn lead(mut self, value: ConvertLeadsResponseLead) -> Self {
        self.lead = Some(value);
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead`](ConvertLeadsResponseBuilder::lead)
    /// - [`partner_id`](ConvertLeadsResponseBuilder::partner_id)
    pub fn build(self) -> Result<ConvertLeadsResponse, BuildError> {
        Ok(ConvertLeadsResponse {
            lead: self.lead.ok_or_else(|| BuildError::missing_field("lead"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
