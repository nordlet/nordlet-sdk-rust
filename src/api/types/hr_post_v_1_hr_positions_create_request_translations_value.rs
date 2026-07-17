pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrPositionsCreateRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1HrPositionsCreateRequestTranslationsValue {
    pub fn builder() -> PostV1HrPositionsCreateRequestTranslationsValueBuilder {
        <PostV1HrPositionsCreateRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsCreateRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1HrPositionsCreateRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsCreateRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrPositionsCreateRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsCreateRequestTranslationsValue, BuildError> {
        Ok(PostV1HrPositionsCreateRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
