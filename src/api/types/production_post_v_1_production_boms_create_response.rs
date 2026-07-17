pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsCreateResponse {
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
    pub lines: Vec<PostV1ProductionBomsCreateResponseLinesItem>,
}

impl PostV1ProductionBomsCreateResponse {
    pub fn builder() -> PostV1ProductionBomsCreateResponseBuilder {
        <PostV1ProductionBomsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    finished_item_id: Option<String>,
    output_quantity: Option<String>,
    is_active: Option<bool>,
    lines: Option<Vec<PostV1ProductionBomsCreateResponseLinesItem>>,
}

impl PostV1ProductionBomsCreateResponseBuilder {
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

    pub fn lines(mut self, value: Vec<PostV1ProductionBomsCreateResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionBomsCreateResponseBuilder::id)
    /// - [`code`](PostV1ProductionBomsCreateResponseBuilder::code)
    /// - [`name`](PostV1ProductionBomsCreateResponseBuilder::name)
    /// - [`finished_item_id`](PostV1ProductionBomsCreateResponseBuilder::finished_item_id)
    /// - [`output_quantity`](PostV1ProductionBomsCreateResponseBuilder::output_quantity)
    /// - [`is_active`](PostV1ProductionBomsCreateResponseBuilder::is_active)
    /// - [`lines`](PostV1ProductionBomsCreateResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1ProductionBomsCreateResponse, BuildError> {
        Ok(PostV1ProductionBomsCreateResponse {
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
