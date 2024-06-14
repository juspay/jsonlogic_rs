use jsonlogic::{
    apply, partial_apply,
    PartialApplyOutcome::{Ambiguous, Resolved},
};
use serde_json::{json, Value};

#[test]
fn test_1() {
    let logic = json!({
      "and": [
        { ">": [{ "var": "age" }, 18] },
        { "<=": [{ "var": "age" }, 65] },
        { "==": [{ "var": "test" }, "test"] },
        {
          "or": [
            { "==": [{ "var": "status" }, "employed"] },
            { "==": [{ "var": "status" }, "student"] }
          ]
        }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "status": "test",
            })
        ),
        Ok(Resolved(json!(false)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "status": "test",
            })
        ),
        apply(
            &logic,
            &json!({
              "status": "test",
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "status": "student",
              "test": "test",
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "status": "student",
              "test": "test",
              "age": 30
            })
        ),
        Ok(Resolved(json!(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "status": "student",
              "test": "test",
              "age": 30
            })
        ),
        apply(
            &logic,
            &json!({
              "status": "student",
              "test": "test",
              "age": 30
            })
        )
        .map(|val| Resolved(val)),
    );
}

#[test]
fn test_2() {
    let logic = json!({
      "if":  [
        { "==": [{ "var": "role" }, "admin"] },
        { "and": [{ "==": [{ "var": "accessLevel" }, "full"] }, { "==": [{ "var": "active" }, true] }]},
        { "==": [{ "var": "accessLevel" }, "limited"] }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "accessLevel": "limited",
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "role": "user",
              "accessLevel": "limited",
            })
        ),
        Ok(Resolved(json!(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "role": "user",
              "accessLevel": "limited",
            })
        ),
        apply(
            &logic,
            &json!({
              "role": "user",
              "accessLevel": "limited",
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "role": "admin",
              "test": "test",
              "age": 30
            })
        ),
        Ok(Ambiguous)
    );
}

#[test]
fn test_3() {
    let logic = json!({
      "and": [
        { "in": [{ "var": "department" }, ["HR", "Engineering", "Marketing"]] },
        {
          "or": [
            { "==": [{ "var": "status" }, "active"] },
            { "<": [{ "var": "experience" }, 5] }
          ]
        }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "department": "HR",
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "department": "HR",
              "status": "active"
            })
        ),
        Ok(Resolved(json!(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "department": "HR",
              "status": "active"
            })
        ),
        apply(
            &logic,
            &json!({
              "department": "HR",
              "status": "active"
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "department": "HR",
              "experience": 3
            })
        ),
        Ok(Resolved(json!(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "department": "HR",
              "experience": 3
            })
        ),
        apply(
            &logic,
            &json!({
              "department": "HR",
              "experience": 3
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "role": "admin",
              "experience": 3
            })
        ),
        Ok(Ambiguous)
    );
}

#[test]
fn test_4() {
    let logic = json!({
      "or": [
        {
          "and": [
            { ">=": [{ "var": "salary" }, 50000] },
            { "<=": [{ "var": "salary" }, 100000] }
          ]
        },
        { "==": [{ "var": "position" }, "intern"] }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "salary": 75000,
            })
        ),
        Ok(Resolved(Value::Bool(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "salary": 75000,
            })
        ),
        apply(
            &logic,
            &json!({
              "salary": 75000,
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "position": "intern",
            })
        ),
        Ok(Resolved(json!(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "position": "intern",
            })
        ),
        apply(
            &logic,
            &json!({
              "position": "intern",
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "position": "admin",
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "salary": 75000,
              "position": "admin",
            })
        ),
        Ok(Resolved(Value::Bool(true)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "salary": 75000,
              "position": "admin",
            })
        ),
        apply(
            &logic,
            &json!({
              "salary": 75000,
              "position": "admin",
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "role": "admin",
              "age": 30
            })
        ),
        Ok(Ambiguous)
    );
}

#[test]
fn test_5() {
    let logic = json!({
      "and": [
        { ">": [{ "var": "temperature" }, 0] },
        { "<=": [{ "var": "temperature" }, 100] },
        {
          "if": [
            { ">": [{ "var": "temperature" }, 50] },
            "Hot",
            "Cold"
          ]
        }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "temperature": 60,
            })
        ),
        Ok(Resolved(json!("Hot")))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "temperature": 60,
            })
        ),
        apply(
            &logic,
            &json!({
              "temperature": 60,
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "role": "user",
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "temperature": 160,
            })
        ),
        Ok(Resolved(json!(false)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "temperature": 160,
            })
        ),
        apply(
            &logic,
            &json!({
              "temperature": 160,
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "temperature": 40,
            })
        ),
        Ok(Resolved(json!("Cold")))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "temperature": 40,
            })
        ),
        apply(
            &logic,
            &json!({
              "temperature": 40,
            })
        )
        .map(|val| Resolved(val)),
    );
}

#[test]
fn test_6() {
    let logic = json!({
      "+": [
        { "*": [{ "var": "x" }, 2] },
        { "/": [{ "var": "y" }, 3] }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "x": 10,
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "y": 10,
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "x": 10,
              "y": 10,
            })
        ),
        Ok(Resolved(json!(23.333333333333332)))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "x": 10,
              "y": 10,
            })
        ),
        apply(
            &logic,
            &json!({
              "x": 10,
              "y": 10,
            })
        )
        .map(|val| Resolved(val)),
    );
}

#[test]
fn test_7() {
    let logic = json!({
      "merge": [
        { "var": "list1" },
        { "var": "list2" }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "list1": ["12", "12wd"],
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "list2": ["12", "12wd"],
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "list1": ["12", "12wd"],
              "list2": ["13", "12wd"],
            })
        ),
        Ok(Resolved(json!(["12", "12wd", "13", "12wd"])))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "list1": ["12", "12wd"],
              "list2": ["13", "12wd"],
            })
        ),
        apply(
            &logic,
            &json!({
              "list1": ["12", "12wd"],
              "list2": ["13", "12wd"],
            })
        )
        .map(|val| Resolved(val)),
    );
}

#[test]
fn test_8() {
    let logic = json!({
      "filter": [
        { "var": "people" },
        { "==": [{ "var": "age" }, 30] }
      ]
    });
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "people": [
                { "name": "Alice", "age": 30 },
                { "name": "Bob", "age": 25 },
                { "name": "Charlie", "age": 30 }
              ]
            })
        ),
        Ok(Resolved(json!([
          { "name": "Alice", "age": 30 },
          { "name": "Charlie", "age": 30 }
        ])))
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "people": [
                { "name": "Alice", "age": 30 },
                { "name": "Bob", "age": 25 },
                { "name": "Charlie", "age": 30 }
              ]
            })
        ),
        apply(
            &logic,
            &json!({
              "people": [
                { "name": "Alice", "age": 30 },
                { "name": "Bob", "age": 25 },
                { "name": "Charlie", "age": 30 }
              ]
            })
        )
        .map(|val| Resolved(val)),
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "people": [
                { "name": "Alice", "age": 30 },
                { "name": "Bob"},
                { "name": "Charlie", "age": 30 }
              ]
            })
        ),
        Ok(Ambiguous)
    );
    assert_eq!(
        partial_apply(
            &logic,
            &json!({
              "list1": ["12", "12wd"],
              "list2": ["13", "12wd"],
            })
        ),
        Ok(Ambiguous)
    );
}
