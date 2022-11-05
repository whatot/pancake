#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_serde_json() {
    let point = Point { x: 3, y: 4 };
    let serialized = serde_json::to_string(&point).unwrap();
    let expected = r#"{"x":3,"y":4}"#;
    assert_eq!(serialized, expected);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, point);
}
