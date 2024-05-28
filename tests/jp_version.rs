use jsonlogic::apply;
use serde_json::{json, Value};

// Juspay version equal
#[test]
fn jp_ver_eq() {
    assert_eq!(
        apply(
            &json!({"jp_ver_eq": [
            "2.0.4-rc.12",
            "2.0.3-rc.89"]}),
            &Value::Null
        ),
        Ok(json!(false))
    );
    assert_eq!(
        apply(&json!({"jp_ver_eq": ["1.0.3", "1.0.3"]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_eq": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.3"})
        ),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_eq": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.2"})
        ),
        Ok(json!(false))
    );
}

// Juspay version greater than
#[test]
fn jp_ver_gt() {
    assert_eq!(
        apply(
            &json!({"jp_ver_gt": [
            "2.0.4-rc.12",
            "2.0.3-rc.89"]}),
            &Value::Null
        ),
        Ok(json!(true))
    );
    assert_eq!(
        apply(&json!({"jp_ver_gt": ["1.0.4", "1.0.3"]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_gt": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.3"})
        ),
        Ok(json!(false))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_gt": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.4"})
        ),
        Ok(json!(true))
    );
}

// Juspay version greater than or equal
#[test]
fn jp_ver_ge() {
    assert_eq!(
        apply(
            &json!({"jp_ver_ge": [
            "2.0.4-rc.12",
            "2.0.3-rc.89"]}),
            &Value::Null
        ),
        Ok(json!(true))
    );
    assert_eq!(
        apply(&json!({"jp_ver_ge": ["1.0.4", "1.0.3"]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_ge": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.3"})
        ),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_ge": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.4"})
        ),
        Ok(json!(true))
    );
}

// Juspay version lesser than
#[test]
fn jp_ver_lt() {
    assert_eq!(
        apply(
            &json!({"jp_ver_lt": [
            "2.0.4-rc.12",
            "2.0.3-rc.89"]}),
            &Value::Null
        ),
        Ok(json!(false))
    );
    assert_eq!(
        apply(&json!({"jp_ver_lt": ["1.0.2", "1.0.3"]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_lt": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.3"})
        ),
        Ok(json!(false))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_lt": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.2"})
        ),
        Ok(json!(true))
    );
}

// Juspay version lesser than or equal
#[test]
fn jp_ver_le() {
    assert_eq!(
        apply(
            &json!({"jp_ver_le": [
            "2.0.4-rc.12",
            "2.0.3-rc.89"]}),
            &Value::Null
        ),
        Ok(json!(false))
    );
    assert_eq!(
        apply(&json!({"jp_ver_le": ["1.0.2", "1.0.3"]}), &Value::Null),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_le": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.3"})
        ),
        Ok(json!(true))
    );
    assert_eq!(
        apply(
            &json!({"jp_ver_le": [{"var": "version"}, "1.0.3"]}),
            &json!({"version": "1.0.2"})
        ),
        Ok(json!(true))
    );
}
