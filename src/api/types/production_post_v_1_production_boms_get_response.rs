pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "finishedItemId")]
    #[serde(default)]
    pub finished_item_id: String,
    #[serde(rename = "outputQuantity")]
    #[serde(default)]
    pub output_quantity: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub lines: Vec<PostV1ProductionBomsGetResponseLinesItem>,
}

impl PostV1ProductionBomsGetResponse {
    pub fn builder() -> PostV1ProductionBomsGetResponseBuilder {
        <PostV1ProductionBomsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsGetResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    finished_item_id: Option<String>,
    output_quantity: Option<String>,
    is_active: Option<bool>,
    lines: Option<Vec<PostV1ProductionBomsGetResponseLinesItem>>,
}

impl PostV1ProductionBomsGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<PostV1ProductionBomsGetResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionBomsGetResponseBuilder::id)
    /// - [`code`](PostV1ProductionBomsGetResponseBuilder::code)
    /// - [`name`](PostV1ProductionBomsGetResponseBuilder::name)
    /// - [`finished_item_id`](PostV1ProductionBomsGetResponseBuilder::finished_item_id)
    /// - [`output_quantity`](PostV1ProductionBomsGetResponseBuilder::output_quantity)
    /// - [`is_active`](PostV1ProductionBomsGetResponseBuilder::is_active)
    /// - [`lines`](PostV1ProductionBomsGetResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1ProductionBomsGetResponse, BuildError> {
        Ok(PostV1ProductionBomsGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            finished_item_id: self
                .finished_item_id
                .ok_or_else(|| BuildError::missing_field("finished_item_id"))?,
            output_quantity: self
                .output_quantity
                .ok_or_else(|| BuildError::missing_field("output_quantity"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
