use nanoserde::DeJson;

#[derive(DeJson)]
pub struct Test {
    pub a: f32,
    pub b: f32,
    c: Option<String>,
    d: Option<String>,
}

fn main() {
    let json = r#"{
        "a": 1,
        "b": 2.0,
        "d": "hello"
    }"#;

    let test: Test = DeJson::deserialize_json(json).unwrap();
    assert_eq!(test.a, 1.);
}
