pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "finishedItemId")]
    #[serde(default)]
    pub finished_item_id: String,
    #[serde(rename = "outputQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_quantity: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1ProductionBomsCreateRequestLinesItem>,
}

impl PostV1ProductionBomsCreateRequest {
    pub fn builder() -> PostV1ProductionBomsCreateRequestBuilder {
        <PostV1ProductionBomsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    finished_item_id: Option<String>,
    output_quantity: Option<String>,
    lines: Option<Vec<PostV1ProductionBomsCreateRequestLinesItem>>,
}

impl PostV1ProductionBomsCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn finished_item_id(mut self, value: impl Into<String>) -> Self {
        self.finished_item_id = Some(value.into());
        self
    }

    pub fn output_quantity(mut self, value: impl Into<String>) -> Self {
        self.output_quantity = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1ProductionBomsCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ProductionBomsCreateRequestBuilder::code)
    /// - [`name`](PostV1ProductionBomsCreateRequestBuilder::name)
    /// - [`finished_item_id`](PostV1ProductionBomsCreateRequestBuilder::finished_item_id)
    /// - [`lines`](PostV1ProductionBomsCreateRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1ProductionBomsCreateRequest, BuildError> {
        Ok(PostV1ProductionBomsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            finished_item_id: self
                .finished_item_id
                .ok_or_else(|| BuildError::missing_field("finished_item_id"))?,
            output_quantity: self.output_quantity,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
