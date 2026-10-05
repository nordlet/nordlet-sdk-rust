pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GlDetailReportsResponseAccount {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
}

impl GlDetailReportsResponseAccount {
    pub fn builder() -> GlDetailReportsResponseAccountBuilder {
        <GlDetailReportsResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GlDetailReportsResponseAccountBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
}

impl GlDetailReportsResponseAccountBuilder {
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

    /// Consumes the builder and constructs a [`GlDetailReportsResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GlDetailReportsResponseAccountBuilder::code)
    /// - [`name`](GlDetailReportsResponseAccountBuilder::name)
    /// - [`r#type`](GlDetailReportsResponseAccountBuilder::r#type)
    pub fn build(self) -> Result<GlDetailReportsResponseAccount, BuildError> {
        Ok(GlDetailReportsResponseAccount {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
