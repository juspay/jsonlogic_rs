use super::jp_version::compare_version;
use super::{logic, Data, Expression};

use serde_json::{json, Value};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let first = match args.get(0) {
        Some(arg) => logic::coerce_to_str(&arg.compute(data)),
        None => return json!(false),
    };

    let second = match args.get(1) {
        Some(arg) => logic::coerce_to_str(&arg.compute(data)),
        None => return json!(false),
    };

    let result = compare_version(&first, &second, true);

    Value::Bool(result.is_eq())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn equal() {
        assert_eq!(compute_const!(), Value::Bool(false));
        assert_eq!(compute_const!(json!("test")), Value::Bool(false));
        assert_eq!(compute_const!(json!("1.0.3")), Value::Bool(false));
        assert_eq!(
            compute_const!(json!("1.0.3"), json!("test")),
            Value::Bool(false)
        );
        assert_eq!(
            compute_const!(json!("1.0.3"), json!("1.0.2")),
            Value::Bool(false)
        );
        assert_eq!(
            compute_const!(json!("1.0.3"), json!("1.0.3")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.2"), json!("1.0.3")),
            Value::Bool(false)
        );
    }
}
