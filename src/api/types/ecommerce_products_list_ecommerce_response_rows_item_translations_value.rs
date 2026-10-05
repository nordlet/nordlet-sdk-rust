pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductsListEcommerceResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ProductsListEcommerceResponseRowsItemTranslationsValue {
    pub fn builder() -> ProductsListEcommerceResponseRowsItemTranslationsValueBuilder {
        <ProductsListEcommerceResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductsListEcommerceResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ProductsListEcommerceResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductsListEcommerceResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ProductsListEcommerceResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(
        self,
    ) -> Result<ProductsListEcommerceResponseRowsItemTranslationsValue, BuildError> {
        Ok(ProductsListEcommerceResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
