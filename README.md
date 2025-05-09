# jsonlogic_rs &emsp; [![Build Status]][github] [![Latest Version]][crates.io]

[Build Status]: https://github.com/marvindv/jsonlogic_rs/workflows/build/badge.svg?branch=master
[github]: https://github.com/marvindv/jsonlogic_rs
[Latest Version]: https://img.shields.io/crates/v/jsonlogic.svg
[crates.io]: https://crates.io/crates/jsonlogic

**A [JsonLogic](http://jsonlogic.com/) implementation in Rust.**

To use this library, add

```toml
[dependencies]
jsonlogic = "0.5"
```

to your `Cargo.toml`.

## Usage

```rust
use serde_json::{json, Value};

let rule = json!({
    "===": [
        2,
        { "var": "foo" }
    ]
});

let data = json!({ "foo": 2 });
assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(true)));

let data = json!({ "foo": 3 });
assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(false)));
```

See the [`examples`](https://github.com/marvindv/jsonlogic_rs/tree/master/examples) directory for more usage examples.

## Operations

**jsonlogic_rs** supports all JsonLogic operations. For detailed informations about all operations and their arguments, head over to [Supported Operations](http://jsonlogic.com/operations.html) on
[jsonlogic.com](http://jsonlogic.com/).

For Rust usage examples and edge cases have a look at the linked tests for each operator below.

* Accessing Data
    - [`var`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/data_access.rs#L4)
    - [`missing`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/data_access.rs#L89)
    - [`missing_some`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/data_access.rs#L117)
* Logic and Boolean Operations
    - [`if`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L4)
    - [`==`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L20)
    - [`===`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L76)
    - [`!=`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L128)
    - [`!==`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L190)
    - [`!`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L249)
    - [`!!`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L272)
    - [`or`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L294)
    - [`and`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/logic_and_boolean.rs#L383)
* Numeric Operations
    - [`>`, `>=`, `<`, and `<=`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L5)
    - Between ([exclusive](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L25), [inclusive](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L41))
    - [`max`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L62) and [`min`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L57)
    - Arithmetic, [`+`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L74) [`-`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L84) [`*`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L91) [`/`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L104)
    - [`%`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/numeric.rs#L118)
* Array Operations
    - [`map`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L52), [`reduce`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L94) and [`filter`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L69)
    - [`all`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L134), [`none`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L237) and [`some`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L179)
    - [`merge`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L5)
    - [`in`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/array.rs#L41)
* String Operations
    - [`in`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/string.rs#L4)
    - [`cat`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/string.rs#L18)
    - [`substr`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/string.rs#L35)
    - [`match`](https://github.com/juspay/jsonlogic_rs/blob/master/tests/string.rs#L56)
* Miscellaneous
    - [`log`](https://github.com/marvindv/jsonlogic_rs/blob/master/tests/misc.rs#L5)
* Juspay Version Comparision
    - [`jp_ver_eq`](https://github.com/juspay/jsonlogic_rs/blob/master/tests/jp_version.rs#L4)
    - [`jp_ver_gt`](https://github.com/juspay/jsonlogic_rs/blob/master/tests/jp_version.rs#L27)
    - [`jp_ver_ge`](https://github.com/juspay/jsonlogic_rs/blob/master/tests/jp_version.rs#L50)
    - [`jp_ver_lt`](https://github.com/juspay/jsonlogic_rs/blob/master/tests/jp_version.rs#L73)
    - [`jp_ver_le`](https://github.com/juspay/jsonlogic_rs/blob/master/tests/jp_version.rs#L96)

## Validation

The library now includes a validation module to ensure JSON Logic rules conform to your requirements:

```rust
use jsonlogic::validation::{ValidationConfig, validate, allowed_operators, variable_set, RequireAndWrapper};
use serde_json::json;

// Create a validation configuration
let config = ValidationConfig {
    require_and_wrapper: Some(RequireAndWrapper { allow_empty: true }),
};

// Validate a rule
let rule = json!({
    "and": [
        {"==": [{"var": "age"}, 18]},
        {"==": [{"var": "name"}, Joe]},
    ]
});

match validate(&rule, &config) {
    Ok(_) => println!("Rule is valid!"),
    Err(err) => println!("Validation failed: {} at {}", err.message, err.path),
}
```

The validation module lets you:
- Require rules to be wrapped in an 'and' block (with option to allow empty rules)

Future versions will include more validation options, such as:
- Restrict which operators can be used
- Limit the depth of the rule's expression tree
- Control which variables can be accessed
- Ensure required variables are present
- Apply custom validation logic
