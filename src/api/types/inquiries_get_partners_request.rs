pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InquiriesGetPartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl InquiriesGetPartnersRequest {
    pub fn builder() -> InquiriesGetPartnersRequestBuilder {
        <InquiriesGetPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InquiriesGetPartnersRequestBuilder {
    id: Option<String>,
}

impl InquiriesGetPartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InquiriesGetPartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InquiriesGetPartnersRequestBuilder::id)
    pub fn build(self) -> Result<InquiriesGetPartnersRequest, BuildError> {
        Ok(InquiriesGetPartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
