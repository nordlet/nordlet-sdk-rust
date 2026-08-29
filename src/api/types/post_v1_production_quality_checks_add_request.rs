pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionQualityChecksAddRequest {
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionQualityChecksAddRequest {
    pub fn builder() -> PostV1ProductionQualityChecksAddRequestBuilder {
        <PostV1ProductionQualityChecksAddRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionQualityChecksAddRequestBuilder {
    order_id: Option<String>,
    name: Option<String>,
    notes: Option<String>,
}

impl PostV1ProductionQualityChecksAddRequestBuilder {
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionQualityChecksAddRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](PostV1ProductionQualityChecksAddRequestBuilder::order_id)
    /// - [`name`](PostV1ProductionQualityChecksAddRequestBuilder::name)
    pub fn build(self) -> Result<PostV1ProductionQualityChecksAddRequest, BuildError> {
        Ok(PostV1ProductionQualityChecksAddRequest {
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            notes: self.notes,
        })
    }
}
