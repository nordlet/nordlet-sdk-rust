pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrPositionsCreateResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1HrPositionsCreateResponseTranslationsValue {
    pub fn builder() -> PostV1HrPositionsCreateResponseTranslationsValueBuilder {
        <PostV1HrPositionsCreateResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsCreateResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1HrPositionsCreateResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsCreateResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrPositionsCreateResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsCreateResponseTranslationsValue, BuildError> {
        Ok(PostV1HrPositionsCreateResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
