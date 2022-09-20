use indexmap::IndexMap;
use openapiv3::{Components, OpenAPI, PathItem, ReferenceOr};

#[derive(Debug, Clone)]
pub struct Spec(OpenAPI);

impl Spec {
    pub fn new(spec: OpenAPI) -> Self {
        Self(spec)
    }

    fn components(&self) -> &Components {
        self.0.components.as_ref().expect("Spec did not contain `components`!")
    }

    /// Return an iterator over the paths to each endpoint
    pub fn paths(&self) -> impl Iterator<Item = &String> {
        self.0.paths.paths.keys()
    }

    pub fn component_schemas(&self) -> &IndexMap<String, ReferenceOr<openapiv3::Schema>> {
        &self.components().schemas
    }

    pub fn get_schema_unchecked(&self, name: &str) -> &ReferenceOr<openapiv3::Schema> {
        self.component_schemas()
            .get(name)
            .as_ref()
            .unwrap_or_else(|| panic!("Expected to find a schema with name = {}", name))
    }

    pub fn get_request_unchecked(&self, path: &str) -> &ReferenceOr<PathItem> {
        self.0.paths.paths.get(path).expect("Path not found")
    }
}
