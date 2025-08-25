use oneof::OneOf;

#[derive(OneOf)]
struct TestStruct {
    a: Option<String>,
    b: Option<i32>,
    c: Option<f64>,
}

#[test]
fn test_validate_oneof() {
    let valid_instance = TestStruct {
        a: Some("Hello".to_string()),
        b: None,
        c: None,
    };
    assert!(valid_instance.validate_oneof().is_ok());

    let invalid_instance = TestStruct {
        a: Some("Hello".to_string()),
        b: Some(42),
        c: None,
    };
    assert!(invalid_instance.validate_oneof().is_err());

    let empty_instance = TestStruct {
        a: None,
        b: None,
        c: None,
    };
    assert!(empty_instance.validate_oneof().is_err());
}

#[test]
fn test_oneof_count() {
    let instance = TestStruct {
        a: Some("Hello".to_string()),
        b: Some(42),
        c: None,
    };
    assert_eq!(instance.oneof_count(), 2);
}
