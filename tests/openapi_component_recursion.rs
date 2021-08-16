#![cfg(feature = "openapi")]

use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Schema)]
#[schema(component = "Bar")]
pub struct Bar {
    pub foo: Box<Foo>,
}

#[derive(Debug, Deserialize, Serialize, Schema)]
#[schema(component = "Foo")]
pub struct Foo {
    pub bar: Option<Box<Bar>>,
}

#[get("/")]
fn test_r(
    _: Json<Foo>,
) -> String {
    String::new()
}

#[test]
fn test_component_recursion_compile() {
    let (spec, _) = openapi::spec().build(|| test_r());
    let schemas = &spec.components.as_ref().unwrap().schemas;
    println!("{}", serde_yaml::to_string(&schemas).unwrap());
    for (name, _) in schemas {
        assert!(name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-'))
    }
    assert!(schemas.contains_key("Foo"));
    assert!(schemas.contains_key("Bar"));
}
