use serde_json::{Number, Value};

use super::logic;

/// %, finds the remainder after the first argument is divided by the second argument.
pub fn compute(args: &[Value]) -> Value {
    let a = match args.get(0).and_then(|a| logic::coerce_to_f64(a)) {
        Some(a) => a,
        None => return Value::Null,
    };

    let b = match args.get(1).and_then(|b| logic::coerce_to_f64(b)) {
        Some(b) => b,
        None => return Value::Null,
    };

    match Number::from_f64(a % b) {
        Some(num) => Value::Number(num),
        None => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn null() {
        assert_eq!(compute(&[]), Value::Null);
        assert_eq!(compute(&[json!("a")]), Value::Null);
        assert_eq!(compute(&[json!(1)]), Value::Null);
        assert_eq!(compute(&[json!(1), json!(0)]), Value::Null);

        assert_eq!(compute(&[json!(1), json!(2)]), json!(1.0));
        assert_eq!(compute(&[json!(101), json!(2)]), json!(1.0));
        assert_eq!(compute(&[json!(102), json!(2)]), json!(0.0));
    }
}