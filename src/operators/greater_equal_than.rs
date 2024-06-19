use serde_json::Value;

use super::{logic, Data, Expression, PartialResult};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    Value::Bool(logic::greater_equal_than(&a, &b))
}

// early returns on finding either Ambiguous arg
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let a = match args.get(0) {
        Some(arg) => arg.partial_compute(data)?,
        None => return Ok(Value::Bool(false)),
    };

    let b = match args.get(1) {
        Some(arg) => arg.partial_compute(data)?,
        None => return Ok(Value::Bool(false)),
    };

    Ok(Value::Bool(logic::greater_equal_than(&a, &b)))
}
