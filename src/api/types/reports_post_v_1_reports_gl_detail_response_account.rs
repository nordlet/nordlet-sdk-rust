pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGlDetailResponseAccount {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
}

impl PostV1ReportsGlDetailResponseAccount {
    pub fn builder() -> PostV1ReportsGlDetailResponseAccountBuilder {
        <PostV1ReportsGlDetailResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGlDetailResponseAccountBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
}

impl PostV1ReportsGlDetailResponseAccountBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsGlDetailResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReportsGlDetailResponseAccountBuilder::code)
    /// - [`name`](PostV1ReportsGlDetailResponseAccountBuilder::name)
    /// - [`r#type`](PostV1ReportsGlDetailResponseAccountBuilder::r#type)
    pub fn build(self) -> Result<PostV1ReportsGlDetailResponseAccount, BuildError> {
        Ok(PostV1ReportsGlDetailResponseAccount {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
