use serde_json::Value;

use super::{logic, Ambiguous, Data, Expression, PartialResult};

/// Takes an array as the first argument and a condition as the second argument. Returns `true`
/// if the condition evaluates to a falsy value for each element of the first parameter.
///
/// `var` operations inside the second argument expression are relative to the array element
/// being tested.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let arr = match args.get(0).map(|arg| arg.compute(data)) {
        Some(Value::Array(arr)) => arr,
        _ => return Value::Bool(true),
    };
    let condition = match args.get(1) {
        Some(expr) => expr,
        None => return Value::Bool(true),
    };

    for elem in arr.iter() {
        let result = condition.compute(&Data::from_json(&elem));
        if logic::is_truthy(&result) {
            return Value::Bool(false);
        }
    }

    // Condition is truthy for all elements.
    Value::Bool(true)
}

// early returns on finding Ambiguous first arg and for Ambiguous results, returns Ambiguous when no there are no true results
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let arr = match args
        .get(0)
        .map(|arg| arg.partial_compute(data))
        .transpose()?
    {
        Some(Value::Array(arr)) => arr,
        _ => return Ok(Value::Bool(true)),
    };
    let condition = match args.get(1) {
        Some(expr) => expr,
        None => return Ok(Value::Bool(true)),
    };

    let mut is_ambiguous = false;

    for elem in arr.iter() {
        match condition.partial_compute(&Data::from_json(&elem)) {
            Err(Ambiguous) => is_ambiguous = true,
            Ok(result) if logic::is_truthy(&result) => return Ok(Value::Bool(false)),
            _ => (),
        }
    }

    if is_ambiguous {
        return Err(Ambiguous);
    }

    // Condition is truthy for all elements.
    Ok(Value::Bool(true))
}
