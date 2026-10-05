pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilesListPartnersRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl FilesListPartnersRequest {
    pub fn builder() -> FilesListPartnersRequestBuilder {
        <FilesListPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilesListPartnersRequestBuilder {
    partner_id: Option<String>,
}

impl FilesListPartnersRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FilesListPartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](FilesListPartnersRequestBuilder::partner_id)
    pub fn build(self) -> Result<FilesListPartnersRequest, BuildError> {
        Ok(FilesListPartnersRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
