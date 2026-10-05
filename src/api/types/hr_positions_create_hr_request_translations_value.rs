pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PositionsCreateHrRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PositionsCreateHrRequestTranslationsValue {
    pub fn builder() -> PositionsCreateHrRequestTranslationsValueBuilder {
        <PositionsCreateHrRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsCreateHrRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl PositionsCreateHrRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PositionsCreateHrRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PositionsCreateHrRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PositionsCreateHrRequestTranslationsValue, BuildError> {
        Ok(PositionsCreateHrRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
